use opendal::{Operator, services};
use thiserror::Error;

use crate::transfer::{DataSink, DataSource};

pub struct AzureBlobConfig {
    container: String,
    account_name: String,
    account_key: String,
    path: String,
}

impl AzureBlobConfig {
    pub fn new(
        container: String,
        account_name: String,
        account_key: String,
        path: String,
    ) -> Self {
        Self {
            container,
            account_name,
            account_key,
            path,
        }
    }

    pub fn build(self) -> AzureBlobDriver {
        AzureBlobDriver::new(self)
    }
}

pub struct AzureBlobDriver {
    config: AzureBlobConfig,
}

impl AzureBlobDriver {
    fn new(config: AzureBlobConfig) -> Self {
        Self { config }
    }

    fn operator(&self) -> Result<Operator, opendal::Error> {
        let builder = services::Azblob::default()
            .container(&self.config.container)
            .account_name(&self.config.account_name)
            .account_key(&self.config.account_key);

        Ok(Operator::new(builder)?.finish())
    }
}

impl DataSource for AzureBlobDriver {
    type Error = AzureBlobError;

    async fn read(&self) -> Result<Vec<u8>, Self::Error> {
        let op = self.operator()?;
        let path = self.config.path.trim_start_matches('/');
        let bytes = op.read(path).await?;
        Ok(bytes.to_vec())
    }
}

impl DataSink for AzureBlobDriver {
    type Error = AzureBlobError;

    async fn write(&self, bytes: &[u8]) -> Result<(), Self::Error> {
        let op = self.operator()?;
        let path = self.config.path.trim_start_matches('/');
        op.write(path, bytes.to_vec()).await?;
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum AzureBlobError {
    #[error("Azure Blob operation failed: {0}")]
    Operation(#[from] opendal::Error),
}
