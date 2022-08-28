use std::{io::{BufReader, self}, path::Path, fs::File};
use thiserror::Error;
use quick_xml::Reader;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Io error reading xml file")]
    ReadXmlFile(#[source] io::Error)
}

/// Parses Stack Exchange Post XMLs
pub struct PostReader {
    /// We reuse the same piece of memory to read all the events into.
    pub buf: Vec<u8>,
    pub xml_reader: Reader<BufReader<File>>
}

impl PostReader {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, Error> {
        let buf = Vec::new();

        let file = File::open(path).map_err(Error::ReadXmlFile)?;
        let reader = BufReader::new(file);
        let mut xml_reader = Reader::from_reader(reader);
        // Avoid generating empty text events
        xml_reader.trim_text(true);
        Ok(Self { buf, xml_reader })
    }
}