rust
fn work_with_optional_types(pb: &PathBuf) -> Result<MyStruct, Error> {
    if let Some(filestem) = pb.file_stem() {
        if let Some(filestr) = filestem.to_str() {
            return Ok(MyStruct {
                filename: filestr.to_string()
            });
        }
     }
    Err(_)
}
