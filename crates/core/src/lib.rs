use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use xml::reader::{EventReader, XmlEvent};
pub mod lexer;
pub type TF = HashMap<String, usize>;
pub type TFIndex = HashMap<PathBuf, TF>;
pub type DF = HashMap<String, usize>;
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Model {
    df: DF,
    tf_index: TFIndex,
}
/// return `true` if it can be xml parser parsed
fn check_xml_ext(file_path: impl AsRef<Path>) -> bool {
    //校验拓展名
    match file_path.as_ref().extension().and_then(OsStr::to_str) {
        None => false,
        Some("xml") | Some("xhtml") => true,
        Some(_) => {
            eprintln!(
                "the path of {file_path} is not an XML file",
                file_path = file_path.as_ref().display()
            );
            false
        }
    }
}
/// read the path of xml path and convert its string into the return value
/// ___
/// promise that every file path has been checked before
fn read_xml_file(file_path: impl AsRef<Path>) -> std::io::Result<String> {
    let file = File::open(file_path)?;
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
/// tokenize the file content into the `TFIndex`
fn tokenize_file(dir_path: &impl AsRef<Path>, res: &mut Model) -> std::io::Result<()> {
    let content = read_xml_file(dir_path.as_ref())?
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
    res.tf_index_mut()
        .insert(PathBuf::from(dir_path.as_ref()), tf);
    Ok(())
}
/// this fn will recursively watch all the subfiles and accumulate into `res`
fn for_each_file(dir_path: impl AsRef<Path>, res: &mut Model) -> std::io::Result<()> {
    //base case 文件
    if dir_path.as_ref().is_file() {
        let path = dir_path.as_ref();
        if check_xml_ext(path) {
            tokenize_file(&path, res)?;
        }
        return Ok(());
    }
    for file in fs::read_dir(dir_path)? {
        let file = file?;
        let path = file.path();
        println!("Indexing {path:?}", path = path);
        //是目录递归
        if path.is_dir() {
            for_each_file(path.clone(), res)?;
        } else {
            if check_xml_ext(path.clone()) {
                tokenize_file(&path, res)?;
            }
        }
    }
    Ok(())
}
/// 驱动函数，读取给定文件夹，然后解析输出到对应文件,写入`Model`
pub fn read_xml_dir_and_write(
    dir_path: impl AsRef<Path>,
    target_path: impl AsRef<Path>,
) -> std::io::Result<()> {
    let mut data = Model::default();
    for_each_file(dir_path, &mut data)?;
    //build the data to be imported
    let tmp_path = target_path.as_ref();
    println!("Writing Index to {:?}", tmp_path);
    let target_file = File::create(target_path)?;
    serde_json::to_writer(BufWriter::new(target_file), &data)?;
    Ok(())
}

//unused
#[allow(unused)]
fn read_xml_dir(dir_path: impl AsRef<Path>) -> std::io::Result<()> {
    let mut res = Model::default();
    for_each_file(dir_path, &mut res)?;
    for (path, tf) in res.tf_index() {
        println!(
            "{path:?} has {count} uk terms",
            count = res.tf_index().len()
        );
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;

    use super::*;
    #[test]
    fn test_lexer() -> std::io::Result<()> {
        let content = read_xml_file("../../docs.gl/gl4/glClear.xhtml")?
            .chars()
            .collect::<Vec<_>>();
        let lexer = lexer::Lexer::new(&content);
        for token in lexer {
            println!("{token}");
        }
        Ok(())
    }
    #[test]
    fn test_fre() -> std::io::Result<()> {
        read_xml_dir("../../docs.gl/gl4")
    }
    #[test]
    fn it_works() -> std::io::Result<()> {
        const FILE_PATH: &str = "../../docs.gl";
        read_xml_dir(FILE_PATH)?;
        Ok(())
    }
    #[test]
    fn test_stemmer() {
        let vec = "linear linearly".chars().collect::<Vec<_>>();
        let lexer = Lexer::new(&vec);
        for token in lexer {
            println!("{token}")
        }
    }
}

impl Model {
    pub fn new(df: DF, tf_index: TFIndex) -> Self {
        Self { df, tf_index }
    }
    pub fn tf_index(&self) -> &TFIndex {
        &self.tf_index
    }
    pub fn df(&self) -> &DF {
        &self.df
    }
    pub fn tf_index_mut(&mut self) -> &mut TFIndex {
        &mut self.tf_index
    }
    pub fn df_mut(&mut self) -> &mut DF {
        &mut self.df
    }
}
