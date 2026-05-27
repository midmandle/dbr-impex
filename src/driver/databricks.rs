use opendal::{Operator, services};
use thiserror::Error;

use crate::transfer::{DataSink, DataSource};

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

    fn operator(&self) -> Result<Operator, opendal::Error> {
        let builder = services::Dbfs::default()
            .endpoint(&self.config.workspace_url)
            .token(&self.config.auth_token);

        Ok(Operator::new(builder)?.finish())
    }
}

impl DataSource for DatabricksDriver {
    type Error = DatabricksError;

    async fn read(&self) -> Result<Vec<u8>, Self::Error> {
        let op = self.operator()?;
        let path = self.config.dbfs_path.trim_start_matches('/');
        let bytes = op.read(path).await?;
        Ok(bytes.to_vec())
    }
}

impl DataSink for DatabricksDriver {
    type Error = DatabricksError;

    async fn write(&self, bytes: &[u8]) -> Result<(), Self::Error> {
        let op = self.operator()?;
        let path = self.config.dbfs_path.trim_start_matches('/');
        op.write(path, bytes.to_vec()).await?;
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum DatabricksError {
    #[error("Databricks DBFS operation failed: {0}")]
    Operation(#[from] opendal::Error),
}
