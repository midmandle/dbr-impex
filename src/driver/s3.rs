use opendal::{Operator, services};
use thiserror::Error;

use crate::transfer::{DataSink, DataSource};

pub struct S3Config {
    bucket: String,
    region: String,
    access_key_id: String,
    secret_access_key: String,
    path: String,
}

impl S3Config {
    pub fn new(
        bucket: String,
        region: String,
        access_key_id: String,
        secret_access_key: String,
        path: String,
    ) -> Self {
        Self {
            bucket,
            region,
            access_key_id,
            secret_access_key,
            path,
        }
    }

    pub fn build(self) -> S3Driver {
        S3Driver::new(self)
    }
}

pub struct S3Driver {
    config: S3Config,
}

impl S3Driver {
    fn new(config: S3Config) -> Self {
        Self { config }
    }

    fn operator(&self) -> Result<Operator, opendal::Error> {
        let builder = services::S3::default()
            .bucket(&self.config.bucket)
            .region(&self.config.region)
            .access_key_id(&self.config.access_key_id)
            .secret_access_key(&self.config.secret_access_key);

        Ok(Operator::new(builder)?.finish())
    }
}

impl DataSource for S3Driver {
    type Error = S3Error;

    async fn read(&self) -> Result<Vec<u8>, Self::Error> {
        let op = self.operator()?;
        let path = self.config.path.trim_start_matches('/');
        let bytes = op.read(path).await?;
        Ok(bytes.to_vec())
    }
}

impl DataSink for S3Driver {
    type Error = S3Error;

    async fn write(&self, bytes: &[u8]) -> Result<(), Self::Error> {
        let op = self.operator()?;
        let path = self.config.path.trim_start_matches('/');
        op.write(path, bytes.to_vec()).await?;
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum S3Error {
    #[error("S3 operation failed: {0}")]
    Operation(#[from] opendal::Error),
}
