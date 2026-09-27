use crate::file_handler::FileHandler;
use lopdf::Document;
use std::io::{Error, ErrorKind, Result};
use std::path::Path;

pub(crate) struct PdfFileHandler;
impl FileHandler for PdfFileHandler {
    fn extract_text(&self, file_path: &Path) -> Result<String> {
        let doc = Document::load(file_path).map_err(|err| {
            eprintln!(
                "ERROR: load {file_path} {err}",
                file_path = file_path.display()
            );
            Error::new(ErrorKind::NotFound, err)
        })?;
        // 校验是否加密
        if doc.is_encrypted() {
            eprintln!("ERROR: the pdf is encrypted");
            return Err(Error::new(ErrorKind::InvalidData, "the pdf is encrypted"));
        }
        let pages = doc.get_pages();
        let pages: Vec<u32> = pages.keys().cloned().collect();
        let content = doc.extract_text(&pages).map_err(|err| {
            eprintln!("ERROR: read the content of first page,{err}");
            Error::new(ErrorKind::InvalidData, err)
        })?;
        println!(
            "DEBUG: Extracted {} pages of {}",
            pages.len(),
            file_path.display()
        );
        Ok(content)
    }
}
impl PdfFileHandler {
    pub(crate) fn new() -> Self {
        Self
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_extract_pdf() -> Result<()> {
        Ok(())
    }
}
