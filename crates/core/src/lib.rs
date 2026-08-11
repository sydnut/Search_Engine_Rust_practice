use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use xml::reader::{EventReader, XmlEvent};
mod lexer;
type TF = HashMap<String, usize>;
type TFIndex = HashMap<PathBuf, TF>;
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
fn read_xml_file(file_path: impl AsRef<Path>) -> std::io::Result<String> {
    let file = File::open(file_path)?;
    let reader = EventReader::new(file);
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
/// this fn will recursively watch all the subfiles and accumulate into `res`
fn for_each_file(dir_path: impl AsRef<Path>, res: &mut TFIndex) -> std::io::Result<()> {
    //base case 文件
    if dir_path.as_ref().is_file() {
        tokenize_file(&dir_path, res)?;
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

fn tokenize_file(dir_path: &impl AsRef<Path>, res: &mut TFIndex) -> std::io::Result<()> {
    let content = read_xml_file(dir_path.as_ref())?
        .chars()
        .collect::<Vec<_>>();
    let mut tf: TF = TF::new();
    for token in lexer::Lexer::new(&content) {
        let term = token
            .iter()
            .map(|c| c.to_ascii_uppercase())
            .collect::<String>();
        let fre = tf.entry(term).or_insert(0);
        *fre += 1;
    }
    res.insert(PathBuf::from(dir_path.as_ref()), tf);
    Ok(())
}

pub fn read_xml_dir_and_write(
    dir_path: impl AsRef<Path>,
    target_path: impl AsRef<Path>,
) -> std::io::Result<()> {
    let mut res: TFIndex = TFIndex::new();
    for_each_file(dir_path, &mut res)?;

    let tmp_path = target_path.as_ref();
    println!("Writing Index to {:?}", tmp_path);
    let target_file = File::create(target_path)?;
    serde_json::to_writer(target_file, &res)?;

    Ok(())
}

pub fn read_xml_dir(dir_path: impl AsRef<Path>) -> std::io::Result<()> {
    let mut res: TFIndex = TFIndex::new();
    for_each_file(dir_path, &mut res)?;
    for (path, tf) in res {
        println!("{path:?} has {count} uk terms", count = tf.len());
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lexer() -> std::io::Result<()> {
        let content = read_xml_file("../../docs.gl/gl4/glClear.xhtml")?
            .chars()
            .map(|c| c.to_ascii_uppercase())
            .collect::<Vec<_>>();
        let lexer = lexer::Lexer::new(&content);
        for token in lexer {
            println!("{token}", token = token.iter().collect::<String>());
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
}
