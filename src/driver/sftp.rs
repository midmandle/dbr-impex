use opendal::{Operator, services};
use thiserror::Error;

use crate::transfer::{DataSink, DataSource};

pub struct SftpConfig {
    address: String,
    username: String,
    key_path: String,
    path: String,
}

impl SftpConfig {
    pub fn new(address: String, username: String, key_path: String, path: String) -> Self {
        Self {
            address,
            username,
            key_path,
            path,
        }
    }

    pub fn build(self) -> SftpDriver {
        SftpDriver::new(self)
    }
}

pub struct SftpDriver {
    config: SftpConfig,
}

impl SftpDriver {
    fn new(config: SftpConfig) -> Self {
        Self { config }
    }

    fn operator(&self) -> Result<Operator, opendal::Error> {
        let builder = services::Sftp::default()
            .endpoint(&self.config.address)
            .user(&self.config.username)
            .key(&self.config.key_path);

        Ok(Operator::new(builder)?.finish())
    }
}

impl DataSource for SftpDriver {
    type Error = SftpError;

    async fn read(&self) -> Result<Vec<u8>, Self::Error> {
        let op = self.operator()?;
        let path = self.config.path.trim_start_matches('/');
        let bytes = op.read(path).await?;
        Ok(bytes.to_vec())
    }
}

impl DataSink for SftpDriver {
    type Error = SftpError;

    async fn write(&self, bytes: &[u8]) -> Result<(), Self::Error> {
        let op = self.operator()?;
        let path = self.config.path.trim_start_matches('/');
        op.write(path, bytes.to_vec()).await?;
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum SftpError {
    #[error("SFTP operation failed: {0}")]
    Operation(#[from] opendal::Error),
}
