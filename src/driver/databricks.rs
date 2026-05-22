use base64::Engine;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::json;
use thiserror::Error;

use crate::transfer::DataSink;

#[derive(Deserialize)]
struct DbfsCreateSuccessResponse {
    handle: u64,
}

#[derive(Deserialize)]
struct DbfsErrorResponse {
    error_code: String,
    message: String,
}

pub struct DatabricksConfig {
    client: reqwest::blocking::Client,
    workspace_url: String,
    auth_token: String,
    dbfs_path: String,
}

impl DatabricksConfig {
    pub fn new(
        client: reqwest::blocking::Client,
        workspace_url: String,
        auth_token: String,
        dbfs_path: String,
    ) -> Self {
        Self {
            client,
            workspace_url,
            auth_token,
            dbfs_path,
        }
    }

    pub fn build(self) -> DatabricksDriver {
        DatabricksDriver::new(self)
    }
}

pub struct DatabricksDriver {
    config: DatabricksConfig,
}

impl DatabricksDriver {
    fn new(config: DatabricksConfig) -> Self {
        Self { config }
    }

    fn create_dbfs_file(&self) -> Result<u64, <DatabricksDriver as DataSink>::Error> {
        let create_url = format!("{}{}", self.config.workspace_url, "/api/2.0/dbfs/create");
        let create_response = self
            .config
            .client
            .post(create_url)
            .header(
                "Authorization",
                format!("Bearer {}", self.config.auth_token),
            )
            .header("Content-Type", "application/json")
            .body(
                json!({
                    "path": self.config.dbfs_path,
                    "overwrite": false,
                })
                .to_string(),
            )
            .send()?;

        match create_response.status() {
            StatusCode::OK => {
                let body: DbfsCreateSuccessResponse = create_response.json()?;
                Ok(body.handle)
            }
            _ => {
                let body: DbfsErrorResponse = create_response.json()?;
                Err(DatabricksSinkError::CreateError(body.message))
            }
        }
    }

    fn add_block_to_file(
        &self,
        file_handle: u64,
        chunk: &[u8],
    ) -> Result<(), <DatabricksDriver as DataSink>::Error> {
        let base64_data = base64::engine::general_purpose::STANDARD.encode(chunk);
        let add_block_url = format!("{}{}", self.config.workspace_url, "/api/2.0/dbfs/add-block");
        let add_block_response = self
            .config
            .client
            .post(add_block_url)
            .header(
                "Authorization",
                format!("Bearer {}", self.config.auth_token),
            )
            .header("Content-Type", "application/json")
            .body(
                json!({
                    "handle": file_handle,
                    "data": base64_data
                })
                .to_string(),
            )
            .send()?;

        match add_block_response.status() {
            StatusCode::OK => Ok(()),
            _ => {
                let body: DbfsErrorResponse = add_block_response.json()?;
                Err(DatabricksSinkError::BlockAddError(body.message))
            }
        }
    }

    fn close_file(&self, file_handle: u64) -> Result<(), <DatabricksDriver as DataSink>::Error> {
        let close_url = format!("{}{}", self.config.workspace_url, "/api/2.0/dbfs/close");
        let _close_response = self
            .config
            .client
            .post(close_url)
            .header(
                "Authorization",
                format!("Bearer {}", self.config.auth_token),
            )
            .header("Content-Type", "application/json")
            .body(
                json!({
                    "handle": file_handle
                })
                .to_string(),
            )
            .send()?;

        match _close_response.status() {
            StatusCode::OK => Ok(()),
            _ => {
                let body: DbfsErrorResponse = _close_response.json()?;
                Err(DatabricksSinkError::CloseFileError(body.message))
            }
        }
    }
}

impl DataSink for DatabricksDriver {
    type Error = DatabricksSinkError;

    fn write(&self, bytes: &[u8]) -> Result<(), Self::Error> {
        const CHUNK_SIZE: usize = 1024 * 1024;

        let file_handle = self.create_dbfs_file()?;

        for chunk in bytes.chunks(CHUNK_SIZE) {
            self.add_block_to_file(file_handle, chunk)?;
        }

        self.close_file(file_handle)?;

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum DatabricksSinkError {
    #[error("unable to create file on DBFS: {0}")]
    Request(#[from] reqwest::Error),
    #[error("unable to parse JSON: {0}")]
    JsonParse(#[from] serde_json::Error),
    #[error("unable to create new file: {0}")]
    CreateError(String),
    #[error("unable to add new file block: {0}")]
    BlockAddError(String),
    #[error("unable to close file: {0}")]
    CloseFileError(String),
}

#[cfg(test)]
mod test {

    use httpmock::{Method::POST, MockServer};
    use serde_json::json;

    use crate::{driver::databricks::DatabricksConfig, transfer::DataSink};

    #[test]
    fn writes_to_a_databricks_sink() {
        let server = MockServer::start();
        let create_mock = server.mock(|when, then| {
            when.method(POST)
                .path("/api/2.0/dbfs/create")
                .header("Authorization", "Bearer some_token")
                .header("Content-Type", "application/json")
                .json_body(json!({
                    "path": "/mnt/data/test_path/test_file.csv",
                    "overwrite": false,
                }));

            then.status(200)
                .header("Content-Type", "application/json")
                .json_body(json!({"handle": 1234567890}));
        });

        let add_block_mock = server.mock(|when, then| {
            when.method(POST)
                .path("/api/2.0/dbfs/add-block")
                .header("Authorization", "Bearer some_token")
                .header("Content-Type", "application/json")
                .json_body_includes(
                    json!({
                        "handle": 1234567890,
                        //Some data here
                    })
                    .to_string(),
                );

            then.status(200).json_body(json!({}));
        });

        let close_mock = server.mock(|when, then| {
            when.method(POST)
                .path("/api/2.0/dbfs/close")
                .header("Authorization", "Bearer some_token")
                .header("Content-Type", "application/json")
                .json_body(json!({
                    "handle": 1234567890,
                }));

            then.status(200).json_body(json!({}));
        });

        let client = reqwest::blocking::Client::new();
        let workspace_url = server.base_url();
        let auth_token = String::from("some_token");
        let dbfs_path = String::from("/mnt/data/test_path/test_file.csv");

        let dbr_driver =
            DatabricksConfig::new(client, workspace_url, auth_token, dbfs_path).build();

        let data = b"test, data, here";

        dbr_driver.write(data).unwrap();

        create_mock.assert();
        add_block_mock.assert();
        close_mock.assert();
    }

    #[test]
    fn create_dbfs_file_returns_error_message_for_400() {
        let server = MockServer::start();
        let create_mock = server.mock(|when, then| {
            when.method(POST).path("/api/2.0/dbfs/create");

            then.status(400)
                .header("Content-Type", "application/json")
                .json_body(json!({"error_code": "RESOURCE_ALREADY_EXISTS", "message": "resource is already existing"}));
        });

        let client = reqwest::blocking::Client::new();
        let workspace_url = server.base_url();
        let auth_token = String::from("some_token");
        let dbfs_path = String::from("/mnt/data/test_path/already_existing_file.csv");

        let dbr_driver =
            DatabricksConfig::new(client, workspace_url, auth_token, dbfs_path).build();

        let result = dbr_driver.create_dbfs_file();

        let x = result.err().unwrap();

        create_mock.assert();
        assert_eq!(
            x.to_string(),
            "unable to create new file: resource is already existing"
        );
    }

    #[test]
    fn add_block_to_file_returns_error_message_for_400() {
        let server = MockServer::start();
        let add_block_mock = server.mock(|when, then| {
            when.method(POST).path("/api/2.0/dbfs/add-block");

            then.status(400)
                .header("Content-Type", "application/json")
                .json_body(json!({"error_code": "RESOURCE_DOES_NOT_EXIST", "message": "resource does not exist"}));
        });

        let client = reqwest::blocking::Client::new();
        let workspace_url = server.base_url();
        let auth_token = String::from("some_token");
        let dbfs_path = String::from("/mnt/data/test_path/already_existing_file.csv");

        let dbr_driver =
            DatabricksConfig::new(client, workspace_url, auth_token, dbfs_path).build();

        let data = b"test, data, here";

        let result = dbr_driver.add_block_to_file(1234567890, data);

        let x = result.err().unwrap();

        add_block_mock.assert();
        assert_eq!(
            x.to_string(),
            "unable to add new file block: resource does not exist"
        );
    }

    #[test]
    fn close_returns_error_message_for_400() {
        let server = MockServer::start();
        let close_mock = server.mock(|when, then| {
            when.method(POST).path("/api/2.0/dbfs/close");

            then.status(400)
                .header("Content-Type", "application/json")
                .json_body(json!({"error_code": "RESOURCE_DOES_NOT_EXIST", "message": "resource does not exist"}));
        });

        let client = reqwest::blocking::Client::new();
        let workspace_url = server.base_url();
        let auth_token = String::from("some_token");
        let dbfs_path = String::from("/mnt/data/test_path/already_existing_file.csv");

        let dbr_driver =
            DatabricksConfig::new(client, workspace_url, auth_token, dbfs_path).build();

        let result = dbr_driver.close_file(1234567890);

        let x = result.err().unwrap();

        close_mock.assert();
        assert_eq!(
            x.to_string(),
            "unable to close file: resource does not exist"
        );
    }
}
