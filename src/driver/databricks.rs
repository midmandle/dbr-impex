use opendal::{Operator, services};
use thiserror::Error;

use crate::transfer::DataSink;

pub struct DatabricksConfig {
    workspace_url: String,
    auth_token: String,
    dbfs_path: String,
}

impl DatabricksConfig {
    pub fn new(workspace_url: String, auth_token: String, dbfs_path: String) -> Self {
        Self {
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
}

impl DataSink for DatabricksDriver {
    type Error = DatabricksSinkError;

    async fn write(&self, bytes: &[u8]) -> Result<(), Self::Error> {
        let mut builder = services::Dbfs::default();
        builder = builder
            .endpoint(&self.config.workspace_url)
            .token(&self.config.auth_token);

        let op = Operator::new(builder)?.finish();
        let path = self.config.dbfs_path.trim_start_matches('/');
        op.write(path, bytes.to_vec()).await?;

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum DatabricksSinkError {
    #[error("unable to write to DBFS: {0}")]
    Write(#[from] opendal::Error),
}
