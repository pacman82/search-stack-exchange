use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Io error reading xml file")]
    ReadXmlFile(#[source] io::Error),
    #[error("Invalid xml format: {0}")]
    InvalidXml(String),
    #[error("XML input is malformed")]
    MalformedXml(String),
    #[error("Error embedding something against the API {0}")]
    Embedding(String),
}

impl Error {
    pub fn invalid_xml(message: impl Into<String>) -> Self {
        Error::InvalidXml(message.into())
    }
}
