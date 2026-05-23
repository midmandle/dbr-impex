use thiserror::Error;

pub trait DataSource {
    type Error: std::error::Error + 'static;

    fn read(&self) -> Result<Vec<u8>, Self::Error>;
}

pub trait DataSink {
    type Error: std::error::Error + 'static;

    fn write(&self, bytes: &[u8]) -> Result<(), Self::Error>;
}

pub fn transfer(source: &impl DataSource, sink: &impl DataSink) -> Result<(), TransferError> {
    let resource_data = source
        .read()
        .map_err(|e| TransferError::Read(Box::new(e)))?;

    sink.write(&resource_data)
        .map_err(|e| TransferError::Write(Box::new(e)))?;

    Ok(())
}

#[derive(Error, Debug)]
pub enum TransferError {
    #[error("could not read from source: {0}")]
    Read(Box<dyn std::error::Error + 'static>),
    #[error("could not write to sink")]
    Write(Box<dyn std::error::Error + 'static>),
}

#[cfg(test)]
mod test {
    use std::{
        fs::{File, read_dir},
        path::PathBuf,
    };

    use tempdir::TempDir;

    use crate::{driver::local::LocalConfig, transfer::transfer};

    #[test]
    fn transfers_from_source_to_sink() {
        let file_name = "foo.csv";
        let in_tempdir = TempDir::new("in").expect("should be able to create a tempdir: in");
        let in_file_path = in_tempdir.path().join(file_name);
        let _file = File::create(in_file_path.as_path()).expect("should be able to create file");

        let out_tempdir = TempDir::new("out").expect("should be able to create a tempdir: out");
        let out_file_path = out_tempdir.path().join(file_name);

        let local_source_in = LocalConfig::new(in_file_path).build();
        let local_source_out = LocalConfig::new(out_file_path).build();

        transfer(&local_source_in, &local_source_out).expect("should transfer to/from local");

        let mut outdir_contents =
            read_dir(out_tempdir.path()).expect("should be able to read tempdir: out");

        assert!(
            outdir_contents
                .next()
                .is_some_and(|f| f.unwrap().file_name() == file_name)
        );
    }

    #[test]
    fn errors_on_source_failure() {
        let file_name = "foo.csv";
        let in_tempdir = TempDir::new("in").expect("should be able to create a tempdir: in");
        let in_file_path = in_tempdir.path().join(file_name);
        //No file exists on pupose to test error case

        let out_tempdir = TempDir::new("out").expect("should be able to create a tempdir: out");
        let out_file_path = out_tempdir.path().join(file_name);

        let local_source_in = LocalConfig::new(in_file_path).build();
        let local_source_out = LocalConfig::new(out_file_path).build();

        let result = transfer(&local_source_in, &local_source_out);

        assert!(result.is_err());
    }

    #[test]
    fn errors_on_sink_failure() {
        let file_name = "foo.csv";
        let in_tempdir = TempDir::new("in").expect("should be able to create a tempdir: in");
        let in_file_path = in_tempdir.path().join(file_name);
        let _file = File::create(in_file_path.as_path()).expect("should be able to create file");

        let out_file_path = PathBuf::from("non-existant-folder").join(file_name);

        let local_source_in = LocalConfig::new(in_file_path).build();
        let local_source_out = LocalConfig::new(out_file_path).build();

        let result = transfer(&local_source_in, &local_source_out);

        assert!(result.is_err());
    }
}
