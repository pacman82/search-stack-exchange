use atoi::FromRadix10;
use quick_xml::{
    events::{
        attributes::{AttrError, Attribute},
        Event,
    },
    Reader,
};
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
    MalformedXML(String),
}

/// Parses Stack Exchange Post XMLs
pub struct PostReader {
    /// We reuse the same piece of memory to read all the events into.
    buf: Vec<u8>,
    /// XML reader is placed on the first row, after construction
    xml_reader: Reader<BufReader<File>>,
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
            return Err(Error::InvalidXMLFormat(
                "Expected XML Declaration".to_owned(),
            ));
        }
        // Read start of rows collection; e.g. <posts>
        let event = Self::extract_event(&mut xml_reader, &mut buf)?;
        if !matches!(event, Event::Start(_)) {
            return Err(Error::InvalidXMLFormat(
                "Expected XML Start Rows".to_owned(),
            ));
        }
        Ok(Self { buf, xml_reader })
    }

    fn extract_event<'a>(
        xml_reader: &mut Reader<BufReader<File>>,
        buf: &'a mut Vec<u8>,
    ) -> Result<Event<'a>, Error> {
        buf.clear();
        let result = xml_reader.read_event_into(buf);
        match result {
            Ok(event) => Ok(event),
            Err(quick_xml::Error::Io(cause)) => Err(Error::ReadXmlFile(cause)),
            Err(error) => Err(Error::MalformedXML(error.to_string())),
        }
    }

    pub fn next_post(&mut self) -> Result<Option<Post>, Error> {
        let event = Self::extract_event(&mut self.xml_reader, &mut self.buf)?;
        match event {
            Event::Empty(bytes) => {
                let name = bytes.name();
                if name.as_ref() == b"row" {
                    let post = Post::from_attributes(bytes.attributes())?;
                    Ok(Some(post))
                } else {
                    Err(Error::InvalidXMLFormat(format!(
                        "Unexpected tagname in row: {}",
                        String::from_utf8_lossy(name.as_ref())
                    )))
                }
            }
            Event::End(_) => Ok(None),
            _ => Err(Error::InvalidXMLFormat(
                "Unexpected tag. Expected row.".to_owned(),
            )),
        }
    }
}

pub enum Post {
    Question { id: u64, title: String },
    Other,
}

impl Post {
    fn from_attributes<'a>(
        attributes: impl Iterator<Item = Result<Attribute<'a>, AttrError>>,
    ) -> Result<Self, Error> {
        let mut id = None;
        let mut post_type_id = None;
        let mut title = None;

        for attr in attributes {
            let attr = attr?;
            match attr.key.into_inner() {
                b"Id" => id = Some(attr.value),
                b"PostTypeId" => post_type_id = Some(attr.value),
                b"Title" => title = Some(attr.value.clone()),
                _ => (),
            }
        }
        let post_type_id = post_type_id
            .ok_or_else(|| Error::InvalidXMLFormat("Missing post_type_id in Post".to_owned()))?;
        let post = match post_type_id.as_ref() {
            b"1" => {
                let id =
                    id.ok_or_else(|| Error::InvalidXMLFormat("Missing id in Post".to_owned()))?;
                let (id, _) = u64::from_radix_10(&id);
                let title = title.ok_or_else(|| {
                    Error::InvalidXMLFormat("Missing title in Question".to_owned())
                })?;
                Post::Question {
                    id,
                    title: String::from_utf8(title.to_vec()).map_err(|_| {
                        Error::InvalidXMLFormat("Expected UTF-8 encoding".to_owned())
                    })?,
                }
            }
            _ => Post::Other,
        };
        Ok(post)
    }
}

impl From<AttrError> for Error {
    fn from(source: AttrError) -> Self {
        Error::MalformedXML(source.to_string())
    }
}
