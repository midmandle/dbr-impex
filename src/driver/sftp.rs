use thiserror::Error;

use crate::{driver::sftp, transfer::DataSource};

struct SftpConfig {}

struct SftpDriver {
    config: SftpConfig,
}

impl DataSource for SftpDriver {
    type Error = SftpSourceError;

    fn read(&self) -> Result<Vec<u8>, Self::Error> {
        todo!()
    }
}

#[derive(Error, Debug)]
pub enum SftpSourceError {}
