use crate::file_handler::FileHandler;
use file_register_macro::regiser_handler;
use std::io::{BufReader, Error, ErrorKind, Result};
use std::path::Path;
use xml::reader::{EventReader, XmlEvent};
#[regiser_handler(exts=["xml","xhtml"])]
pub(crate) struct XmlFileHandler;
impl FileHandler for XmlFileHandler {
    fn extract_text(&self, file_path: &Path) -> Result<String> {
        let file = std::fs::File::open(file_path)?;
        let reader = EventReader::new(BufReader::new(file));
        let mut buffer = String::new();
        for event in reader.into_iter() {
            let event = event.map_err(|err| {
                eprintln!("XML Read Event Error:{:?}", err);
                Error::new(ErrorKind::InvalidData, err)
            })?;
            if let XmlEvent::Characters(text) = event {
                buffer.push_str(&text);
                buffer.push(' ');
            }
        }
        Ok(buffer)
    }
}
impl XmlFileHandler {
    pub(crate) fn new() -> Self {
        Self
    }
}
