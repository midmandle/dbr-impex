use thiserror::Error;

use crate::transfer::DataSource;

struct FtpConfig {}

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
    fn read(&self) -> Result<Vec<u8>, Self::Error> {
        todo!()
    }
}

#[derive(Error, Debug)]
enum FtpSourceError {}
