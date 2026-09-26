use std::fs::{self,File};
use std::error::Error;
use std::io::{BufWriter, ErrorKind};
use std::ops::Deref;
use std::path::Path;
mod file_process;
pub mod lexer;
pub mod model;
pub use model::*;

use crate::file_process::tokenize_file;
/// 驱动函数，读取给定文件夹，然后解析输出到对应文件,写入`Model`
pub fn read_xml_dir_and_write(
    dir_path: impl AsRef<Path>,
    target_path: impl AsRef<Path>,
) -> std::io::Result<()> {
    let mut data = Model::default();
    file_process::for_each_file(dir_path, &mut data)?;
    //build the data to be imported
    let tmp_path = target_path.as_ref();
    println!("Writing Index to {:?}", tmp_path);
    let target_file = File::create(target_path)?;
    serde_json::to_writer(BufWriter::new(target_file), &data)?;
    Ok(())
}
/// re_index the updated index files
// 返回待修改的批处理操作，不需反复使用读写锁
pub fn re_index(model:&Model) -> Result<Bulk<Update>,Box<dyn Error>>{
    // 寻找修改后的文件time，寻找差异diff->diff(tf)，由tf的不同修改DF
    // 返回待批处理对象
    let index = model.index();
    let mut new_model=Model::default();
    //TODO 暂未新增文件
    for (path,doc) in index{
        let path=path.deref();
        let meta_res = fs::metadata(path);        
        if meta_res.as_ref().is_err_and(|err|{
            err.kind()==ErrorKind::NotFound
        }){
            eprintln!("DEBUG: {} has been removed",path.display());
            //removed file
            continue;
        }
        let sys_ts = meta_res?.modified()?;
        //1 unmodified
        if sys_ts == doc.get_ts(){
            continue;
        }
        //2 mark diff
        tokenize_file(&path,&mut new_model,Some(sys_ts))?;
    }
    if new_model.is_empty(){
        return Ok(Bulk::new());
    }
    //cal diff
    let mut bulk=Bulk::new();
    let new_index=new_model.index();
    //new_index 是 index 的子集        
    for (path,doc) in index{
        //记录删除的文件的缺失数据，已经修改文件的改动（词的新增、修改、删除）对DF缓存的数据同步
        if let Some(new_doc)=new_index.get(path){
            bulk.extend(new_doc.diff(doc,path.clone()));
        }else{
            //删除了文件
            //1 sync df cache
            for (term,_) in doc.get_tf(){
                //update: df[term]-=1
                bulk.push(Update::modify(Modified::DF,term.clone(), -1));
            } 
            //2 remove this file in new index
            //update: remove path 
            bulk.push(Update::remove(path.clone()));
        }
    }
    Ok(bulk)
}
#[cfg(test)]
mod tests {
    use super::lexer::Lexer;
    #[test]
    fn test_stemmer() {
        let vec = "linear linearly".chars().collect::<Vec<_>>();
        let lexer = Lexer::new(&vec);
        for token in lexer {
            println!("{token}")
        }
    }
    #[test]
    fn test_dismissing_error_kind(){
        let path=std::path::Path::new("abc");
        let meta_res = std::fs::metadata(path);        
        if meta_res.is_err_and(|err|{
            err.kind()==std::io::ErrorKind::NotFound
        }){
            eprintln!("DEBUG: {} has been removed",path.display());
        };
    }
}
