use std::{
    fs::File,
    io::{BufReader, Read, Write},
    path::PathBuf,
};

use thiserror::Error;

use crate::transfer::{DataSink, DataSource};

pub struct LocalConfig {
    path: PathBuf,
}

impl LocalConfig {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn build(self) -> LocalDriver {
        LocalDriver::new(self)
    }
}

pub struct LocalDriver {
    config: LocalConfig,
}

impl LocalDriver {
    fn new(config: LocalConfig) -> Self {
        Self { config }
    }
}

impl DataSource for LocalDriver {
    type Error = LocalSourceError;

    async fn read(&self) -> Result<Vec<u8>, Self::Error> {
        let file_path = self.config.path.as_path();
        let file = File::open(file_path)?;

        let bytes: Vec<u8> = BufReader::new(file)
            .bytes()
            .filter_map(|b| b.ok())
            .collect();

        Ok(bytes)
    }
}

#[derive(Error, Debug)]
pub enum LocalSourceError {
    #[error("unable to open file: {0}")]
    Open(#[from] std::io::Error),
}

impl DataSink for LocalDriver {
    type Error = LocalSinkError;

    async fn write(&self, bytes: &[u8]) -> Result<(), Self::Error> {
        let file_path = self.config.path.as_path();
        let mut file = File::create(file_path)?;

        file.write_all(bytes)?;

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum LocalSinkError {
    #[error("unable to create file: {0}")]
    Create(#[from] std::io::Error),
}

#[cfg(test)]
mod test {
    use std::{
        fs::File,
        io::{Read, Write},
    };

    use tempdir::TempDir;

    use crate::{
        driver::local::LocalConfig,
        transfer::{DataSink, DataSource},
    };

    #[tokio::test]
    async fn reads_bytes_from_source() {
        let file_name = "foo.csv";
        let tempdir = TempDir::new("in").expect("should be able to create a tempdir: in");
        let file_path = tempdir.path().join(file_name);
        let mut file = File::create(file_path.as_path()).expect("should be able to create file");
        file.write_all(b"Hello, world!")
            .expect("should be able to write to the file");

        let local_source = LocalConfig::new(file_path).build();
        let bytes = local_source
            .read()
            .await
            .expect("should be able to read from source");

        assert_eq!(bytes, b"Hello, world!");
    }

    #[tokio::test]
    async fn writes_bytes_to_sink() {
        let file_name = "foo.csv";
        let tempdir = TempDir::new("in").expect("should be able to create a tempdir: in");
        let file_path = tempdir.path().join(file_name);

        let local_sink = LocalConfig::new(file_path.clone()).build();
        local_sink
            .write(b"Hello, world!")
            .await
            .expect("should be able to write to sink");

        let bytes: Vec<u8> = File::open(file_path)
            .expect("should be able to open file")
            .bytes()
            .filter_map(|b| b.ok())
            .collect();

        assert_eq!(bytes, b"Hello, world!");
    }
}
