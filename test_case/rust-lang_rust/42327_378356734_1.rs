rust
use failure::Error;
fn work_with_optional_types(pb: &PathBuf) -> Result<MyStruct, Error> {
    Ok({
        title: pb.file_stem?.to_str()?.to_string()
    })
}
