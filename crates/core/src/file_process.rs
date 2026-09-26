use crate::{lexer, model::*};
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use xml::reader::{EventReader, XmlEvent};
/// read the last modified time of the file path
pub(crate) fn read_systime_from_path(file_path: impl AsRef<Path>) -> std::io::Result<SystemTime> {
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
/// return `true` if it can be xml parser parsed
pub(crate) fn check_xml_ext(file_path: impl AsRef<Path>) -> bool {
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
/// read the path of xml path and convert its `string` into the str split by space `"\ "`
/// ___
/// promise that every file path has been checked before
pub(crate) fn convert_xml_file(file_path: impl AsRef<Path>) -> std::io::Result<String> {
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
/// tokenize the file content and put it in the `Index`
/// > sys_ts will call an sys_call to get itself when it is `None`
pub(crate) fn tokenize_file(
    dir_path: &impl AsRef<Path>,
    res: &mut Model,
    sys_ts: Option<SystemTime>,
) -> std::io::Result<()> {
    let content = convert_xml_file(dir_path.as_ref())?
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
/// this fn will recursively watch all the subfiles and accumulate into `res`
pub fn for_each_file(dir_path: impl AsRef<Path>, res: &mut Model) -> std::io::Result<()> {
    //base case 文件
    if dir_path.as_ref().is_file() {
        let path = dir_path.as_ref();
        if check_xml_ext(path) {
            tokenize_file(&path, res, None)?;
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
                tokenize_file(&path, res, None)?;
            }
        }
    }
    Ok(())
}

mod tests {
    #[test]
    fn test_read_meta_data() -> std::io::Result<()> {
        let path = "test_meta_data_file.txt";
        let _ = std::fs::write(path, "ferris🦀")?;
        println!(
            "the last modified time:{:?}",
            super::read_systime_from_path(path)?
        );
        std::fs::remove_file(path)
    }
    #[test]
    fn test_lexer() -> std::io::Result<()> {
        let content = super::convert_xml_file("../../docs.gl/gl4/glClear.xhtml")?
            .chars()
            .collect::<Vec<_>>();
        let lexer = super::lexer::Lexer::new(&content);
        for token in lexer {
            println!("{token}");
        }
        Ok(())
    }
    #[test]
    fn test_walk_dir() {
        for entry in walkdir::WalkDir::new("../") {
            let entry = entry.unwrap();
            println!("{}", entry.path().display())
        }
        assert_eq!(
            std::path::Path::new(".\\src"),
            std::path::Path::new("./src")
        )
    }
}
