use opendal::{Operator, services};
use thiserror::Error;

use crate::{driver::sftp, transfer::DataSource};

struct SftpConfig {
    address: String,
    username: String,
    key_path: String,
    path: String,
}

impl SftpConfig {
    fn new(address: String, username: String, key_path: String, path: String) -> Self {
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

struct SftpDriver {
    config: SftpConfig,
}

impl SftpDriver {
    fn new(config: SftpConfig) -> Self {
        Self { config }
    }
}

impl DataSource for SftpDriver {
    type Error = SftpSourceError;

    async fn read(&self) -> Result<Vec<u8>, Self::Error> {
        let mut builder = services::Sftp::default();

        builder = builder
            .endpoint(&self.config.address)
            .user(&self.config.username)
            .key(&self.config.key_path);

        let op = Operator::new(builder)?.finish();

        let bytes = op.read(&self.config.path).await?;

        Ok(bytes.to_vec())
    }
}

#[derive(Error, Debug)]
pub enum SftpSourceError {
    #[error("unable to read source")]
    Read(#[from] opendal::Error),
}
