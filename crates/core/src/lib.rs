use std::collections::HashMap;
use std::path::{Path, PathBuf};
use xml::reader::{EventReader, XmlEvent};
use std::fs;
use std::fs::File;

mod lexer;
type TF=HashMap<String, usize>;
type TFIndex = HashMap<PathBuf, TF>;
fn read_xml_file(file_path:impl AsRef<Path>)->std::io::Result<String> {
    let file=fs::File::open(file_path)?;
    let reader = EventReader::new(file);
    let mut buffer=String::new();
    for event in reader.into_iter() {
        let event=event.unwrap_or_else(|err|
            {eprintln!("XML Read Event Error:{:?}",err); std::process::exit(1);});
        if let XmlEvent::Characters(text) = event {
            buffer.push_str(&text);
            buffer.push(' ');
        }
    }
    Ok(buffer)
}
pub fn read_xml_dir_and_write(dir_path:impl AsRef<Path>,target_path:impl AsRef<Path>) ->std::io::Result<()>{
    let mut res:TFIndex=TFIndex::new();
    for file in fs::read_dir(dir_path)? {
        let file = file?;
        println!("Indexing {path:?}", path=file.path());
        let content=read_xml_file(file.path())?
            .chars()
            .collect::<Vec<_>>();
        let mut tf:TF = TF::new();
        for token in lexer::Lexer::new(&content) {
            let term = token.iter()
                .map(|c| c.to_ascii_uppercase())
                .collect::<String>();
            let fre = tf.entry(term).or_insert(0);
            *fre+=1;
        }
        let mut tmp = tf.iter().collect::<Vec<_>>();
        tmp.sort_by_key(|(_,f)|*f);
        tmp.reverse();
        res.insert(file.path(),tf);
    }
    
    let tmp_path =target_path.as_ref();
    println!("Writing Index to {:?}", tmp_path);
    let target_file=File::create(target_path)?;
    serde_json::to_writer(target_file,&res)?;

    Ok(())
}
pub fn read_xml_dir(dir_path:impl AsRef<Path>) ->std::io::Result<()>{
    let mut res:TFIndex=TFIndex::new();
    for file in fs::read_dir(dir_path)? {
        let file = file?;
        println!("Indexing {path:?}", path=file.path());
        let content=read_xml_file(file.path())?
            .chars()
            .collect::<Vec<_>>();
        let mut tf:TF = TF::new();
        for token in lexer::Lexer::new(&content) {
            let term = token.iter()
                .map(|c| c.to_ascii_uppercase())
                .collect::<String>();
            let fre = tf.entry(term).or_insert(0);
            *fre+=1;
        }
        let mut tmp = tf.iter().collect::<Vec<_>>();
        tmp.sort_by_key(|(_,f)|*f);
        tmp.reverse();
        res.insert(file.path(),tf);
    }
    for (path,tf) in res{
        println!("{path:?} has {count} uk terms",count = tf.len());
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;
    #[test]
    fn test_lexer()->std::io::Result<()> {
        let content=read_xml_file("../../docs.gl/gl4/glClear.xhtml")?
            .chars()
            .map(|c|c.to_ascii_uppercase())
            .collect::<Vec<_>>();
        let lexer=lexer::Lexer::new(&content);
        for token in lexer {
            println!("{token}", token=token.iter().collect::<String>());
        }
        Ok(())
    }
    #[test]
    fn test_fre()->std::io::Result<()> {
        read_xml_dir("../../docs.gl/gl4")
    }
    #[test]
    fn it_works()->std::io::Result<()>{
        const FILE_PATH:&str="../../docs.gl/gl4";
        for file in fs::read_dir(FILE_PATH)?{
            let file_path = file?.path();
            let content = read_xml_file(&file_path)?;
            println!("{file_path:?} => {size}",size=content.len());
        }
        Ok(())
    }
}
