use crate::{lexer, model::*};
use std::collections::HashMap;
use std::fs;
use std::io::Result;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
mod xml_handler;
pub(crate) use xml_handler::XmlFileHandler;

pub trait FileHandler: Send {
    fn validate(&self, file_path: &Path) -> bool;
    fn convert_content_to_string_spilt_by_space(&mut self, file_path: &Path) -> Result<String>;
}
pub type Handlers = HashMap<String, Box<dyn FileHandler>>;
#[allow(dead_code)]
pub(crate) enum SupportedType {
    Xml,
    Txt,
}
impl SupportedType {
    pub fn get_handler<'a>(
        handlers: &'a mut Handlers,
        path: &Path,
    ) -> Option<&'a mut Box<dyn FileHandler>> {
        let ext = path.extension();
        if ext.is_none() {
            return None;
        }
        let ext = ext.unwrap().to_string_lossy().into_owned();
        handlers.get_mut(&ext)
    }
}
pub fn walk_and_process(
    handlers: &mut Handlers,
    dir_path: impl AsRef<Path>,
    res: &mut Model,
) -> std::io::Result<()> {
    //base case 文件
    if dir_path.as_ref().is_file() {
        let path = dir_path.as_ref();
        if let Some(handler) = SupportedType::get_handler(handlers, path) {
            tokenize(handler.as_mut(), &path, res, None)?;
        } else {
            println!("WARN: Unsupported file:{}", path.display())
        }
        return Ok(());
    }
    for file in fs::read_dir(dir_path)? {
        let file = file?;
        let path = file.path();
        println!("Indexing {path:?}", path = path);
        //是目录递归
        if path.is_dir() {
            walk_and_process(handlers, path.clone(), res)?;
        } else {
            if let Some(handler) = SupportedType::get_handler(handlers, path.as_path()) {
                tokenize(handler.as_mut(), &path, res, None)?;
            } else {
                println!("WARN: Unsupported file:{}", path.display())
            }
        }
    }
    Ok(())
}
/// tokenize the file content and put it in the `Index`
/// > sys_ts will call an sys_call to get itself when it is `None`
pub fn tokenize(
    handler: &mut dyn FileHandler,
    dir_path: &impl AsRef<Path>,
    res: &mut Model,
    sys_ts: Option<SystemTime>,
) -> std::io::Result<()> {
    let content = handler
        .convert_content_to_string_spilt_by_space(dir_path.as_ref())?
        .chars()
        .collect::<Vec<_>>();
    let mut tf: TF = TF::new();
    for token in lexer::Lexer::new(&content) {
        let fre = tf.entry(token).or_insert(0);
        *fre += 1;
    }
    for term in tf.keys() {
        let df = res.df_mut();
        if let Some(fre) = df.get_mut(term) {
            *fre += 1;
        } else {
            df.insert(term.clone(), 1);
        }
    }
    let sys_ts = if sys_ts.is_none() {
        read_systime_from_path(dir_path)?
    } else {
        sys_ts.unwrap()
    };
    res.index_mut()
        .insert(PathBuf::from(dir_path.as_ref()), Doc::new(tf, sys_ts));
    Ok(())
}
fn read_systime_from_path(file_path: impl AsRef<Path>) -> std::io::Result<SystemTime> {
    let meta_data = fs::metadata(file_path.as_ref()).map_err(|err| {
        eprintln!(
            "ERROR: can not read the metadata of {file_path},{err}",
            file_path = file_path.as_ref().display(),
        );
        err
    })?;
    //modified can also read the last created time
    meta_data.modified().map_err(|err| {
        eprintln!(
            "ERROR: can not read the last modified time of {file_path},{err}",
            file_path = file_path.as_ref().display(),
        );
        err
    })
}
