use std::fs::File;
use std::error::Error;
use std::io::BufWriter;
use std::path::Path;
mod file_process;
pub mod lexer;
pub mod model;
pub use model::*;
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
/// re_index the updated index files.
/// > require the file_path is valid
pub fn re_index(model:&mut Model,file_path: &str) -> Result<(),Box<dyn Error>>{
    // TODO 返回待修改的批处理操作，不要反复使用读写锁，先按标准修改实现一遍
    // 寻找修改后的文件time，寻找差异diff->diff(tf)，由tf的不同修改DF
    // 返回待批处理对象
    Ok(())
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
}
