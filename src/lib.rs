use quick_xml::{Reader, events::Event};
use std::{
    fs::File,
    io::{self, BufReader},
    path::Path,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Io error reading xml file")]
    ReadXmlFile(#[source] io::Error),
    #[error("Invalid xml format: {0}")]
    InvalidXMLFormat(String),
    #[error("XML input is malformed")]
    MalformedXML(String)
}

/// Parses Stack Exchange Post XMLs
pub struct PostReader {
    /// We reuse the same piece of memory to read all the events into.
    pub buf: Vec<u8>,
    /// XML reader is placed on the first row, after construction
    pub xml_reader: Reader<BufReader<File>>,
}

impl PostReader {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, Error> {
        let mut buf = Vec::new();

        let file = File::open(path).map_err(Error::ReadXmlFile)?;
        let reader = BufReader::new(file);
        let mut xml_reader = Reader::from_reader(reader);
        // Avoid generating empty text events
        xml_reader.trim_text(true);
        // Read declaration: E.g.: <?xml version="1.0" encoding="utf-8"?>
        let event = Self::extract_event(&mut xml_reader, &mut buf)?;
        if !matches!(event, Event::Decl(_)) {
            return Err(Error::InvalidXMLFormat("Expected XML Declaration".to_owned()))
        }
        // Read start rows event. E.g. <posts>
        let event = Self::extract_event(&mut xml_reader, &mut buf)?;
        if !matches!(event, Event::Start(_)) {
            return Err(Error::InvalidXMLFormat("Expected XML Start Rows".to_owned()))
        }
        Ok(Self { buf, xml_reader })
    }

    fn extract_event<'a>(xml_reader: &mut Reader<BufReader<File>>, buf: &'a mut Vec<u8>) -> Result<Event<'a>, Error> {
        buf.clear();
        let result = xml_reader
            .read_event_into(buf);
        match result {
            Ok(event) => Ok(event),
            Err(quick_xml::Error::Io(cause)) => Err(Error::ReadXmlFile(cause)),
            Err(error) => Err(Error::MalformedXML(error.to_string())),
        }
    }
}
