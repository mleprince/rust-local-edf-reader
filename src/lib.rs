extern crate edf_reader;
extern crate futures;
use positioned_io::ReadAt;


use edf_reader::async_reader::*;
use edf_reader::file_reader::*;
use edf_reader::sync_reader::*;

use futures::future::{err, ok};
use futures::Future;
use std::fs::File;
use std::path::Path;

/**
Init an EDFReader with the path of a local file
*/
pub fn init_async_reader<P: AsRef<Path>>(
    file_path: P,
) -> Box<Future<Item = AsyncEDFReader<LocalFileReader>, Error = std::io::Error>> {
    match LocalFileReader::init(file_path) {
        Ok(file_reader) => Box::new(AsyncEDFReader::init_with_file_reader(file_reader)),
        Err(e) => Box::new(err::<AsyncEDFReader<LocalFileReader>, std::io::Error>(e)),
    }
}

/**
Init an EDFReader with the path of a local file
*/
pub fn init_sync_reader<P: AsRef<Path>>(
    file_path: P,
) -> Result<SyncEDFReader<LocalFileReader>, std::io::Error> {
    let file_reader = LocalFileReader::init(file_path)?;
    SyncEDFReader::init_with_file_reader(file_reader)
}

/**
 * A FileReader for reading local files in blocking or non-blocking way
 */
pub struct LocalFileReader {
    random_access_file: File,
}

impl LocalFileReader {
    /**
     * Init the fileReader with the path of a local file
     */
    pub fn init<P: AsRef<Path>>(file_path: P) -> Result<LocalFileReader, std::io::Error> {
        let file = File::open(file_path)?;
        Ok(LocalFileReader {
            random_access_file: file,
        })
    }
}

impl SyncFileReader for LocalFileReader {
    fn read(&self, offset: u64, length: u64) -> Result<Vec<u8>, std::io::Error> {
        let mut data = vec![0; length as usize];

        self.random_access_file.read_at(offset, &mut data[..])?;

        Ok(data)
    }
}

impl AsyncFileReader for LocalFileReader {
    fn read_async(
        &self,
        offset: u64,
        length: u64,
    ) -> Box<Future<Item = Vec<u8>, Error = std::io::Error> + Send> {
        match self.read(offset, length) {
            Ok(data) => Box::new(ok::<Vec<u8>, std::io::Error>(data)),
            Err(e) => Box::new(err::<Vec<u8>, std::io::Error>(e)),
        }
    }
}
