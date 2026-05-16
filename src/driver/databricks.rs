use thiserror::Error;

use crate::transfer::DataSink;

pub struct DatabricksConfig {}

pub struct DatabricksDriver {
    config: DatabricksConfig,
}

impl DatabricksDriver {
    pub fn new(config: DatabricksConfig) -> Self {
        Self { config }
    }
}

impl DataSink for DatabricksDriver {
    type Error = DatabricksSinkError;

    fn write(&self, bytes: &[u8]) -> Result<(), Self::Error> {
        todo!()
    }
}

#[derive(Error, Debug)]
pub enum DatabricksSinkError {}
