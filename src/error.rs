use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Io error reading xml file")]
    ReadXmlFile(#[source] io::Error),
    #[error("Invalid xml format: {0}")]
    InvalidXMLFormat(String),
    #[error("XML input is malformed")]
    MalformedXML(String),
    #[error("Error embedding something against the API {0}")]
    Embedding(String),
}
