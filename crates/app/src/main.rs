use core::read_xml_dir_and_write;
fn main() -> std::io::Result<()> {
    for file_path in std::env::args().skip(1).take(1) {
        read_xml_dir_and_write(file_path, "index.json")?;
    }
    Ok(())
}
