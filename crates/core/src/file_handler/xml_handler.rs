use crate::file_handler::FileHandler;
use std::ffi::OsStr;
use std::io::{BufReader, Result};
use std::path::Path;
use xml::reader::{EventReader, XmlEvent};
pub(crate) struct XmlFileHandler;
impl FileHandler for XmlFileHandler {
    fn validate(&self, file_path: &Path) -> bool {
        //校验拓展名
        match file_path.extension().and_then(OsStr::to_str) {
            None => false,
            Some("xml") | Some("xhtml") => true,
            Some(_) => {
                eprintln!(
                    "the path of {file_path} is not an XML file",
                    file_path = file_path.display()
                );
                false
            }
        }
    }
    fn convert_content_to_string_spilt_by_space(&mut self, file_path: &Path) -> Result<String> {
        let file = std::fs::File::open(file_path)?;
        let reader = EventReader::new(BufReader::new(file));
        let mut buffer = String::new();
        for event in reader.into_iter() {
            let event = event.unwrap_or_else(|err| {
                eprintln!("XML Read Event Error:{:?}", err);
                std::process::exit(1);
            });
            if let XmlEvent::Characters(text) = event {
                buffer.push_str(&text);
                buffer.push(' ');
            }
        }
        Ok(buffer)
    }
}
impl XmlFileHandler {
    pub fn new() -> Self {
        Self
    }
}
