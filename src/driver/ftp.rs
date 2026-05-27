use opendal::{Operator, services};
use thiserror::Error;

use crate::transfer::DataSource;

struct FtpConfig {
    address: String,
    username: String,
    password: String,
    path: String,
}

impl FtpConfig {
    fn new(address: String, username: String, password: String, path: String) -> Self {
        Self {
            address,
            username,
            password,
            path,
        }
    }

    pub fn build(self) -> FtpDriver {
        FtpDriver::new(self)
    }
}

struct FtpDriver {
    config: FtpConfig,
}

impl FtpDriver {
    fn new(config: FtpConfig) -> Self {
        Self { config }
    }
}

impl DataSource for FtpDriver {
    type Error = FtpSourceError;
    async fn read(&self) -> Result<Vec<u8>, Self::Error> {
        let mut builder = services::Ftp::default();

        builder = builder
            .endpoint(&self.config.address)
            .password(&self.config.password)
            .user(&self.config.username)
            .root(&self.config.path);

        let op = Operator::new(builder)?.finish();

        let bytes = op.read(&self.config.path).await?;

        Ok(bytes.to_vec())
    }
}

#[derive(Error, Debug)]
enum FtpSourceError {
    #[error("unable to read source")]
    Read(#[from] opendal::Error),
}
