use opendal::{Operator, services};
use thiserror::Error;

use crate::transfer::{DataSink, DataSource};

pub struct GcsConfig {
    bucket: String,
    credential: String,
    path: String,
}

impl GcsConfig {
    pub fn new(bucket: String, credential: String, path: String) -> Self {
        Self {
            bucket,
            credential,
            path,
        }
    }

    pub fn build(self) -> GcsDriver {
        GcsDriver::new(self)
    }
}

pub struct GcsDriver {
    config: GcsConfig,
}

impl GcsDriver {
    fn new(config: GcsConfig) -> Self {
        Self { config }
    }

    fn operator(&self) -> Result<Operator, opendal::Error> {
        let builder = services::Gcs::default()
            .bucket(&self.config.bucket)
            .credential(&self.config.credential);

        Ok(Operator::new(builder)?.finish())
    }
}

impl DataSource for GcsDriver {
    type Error = GcsError;

    async fn read(&self) -> Result<Vec<u8>, Self::Error> {
        let op = self.operator()?;
        let path = self.config.path.trim_start_matches('/');
        let bytes = op.read(path).await?;
        Ok(bytes.to_vec())
    }
}

impl DataSink for GcsDriver {
    type Error = GcsError;

    async fn write(&self, bytes: &[u8]) -> Result<(), Self::Error> {
        let op = self.operator()?;
        let path = self.config.path.trim_start_matches('/');
        op.write(path, bytes.to_vec()).await?;
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum GcsError {
    #[error("GCS operation failed: {0}")]
    Operation(#[from] opendal::Error),
}
