rust
compiler/rustc_serialize/src/json.rs
pub fn from_reader(rdr: &mut dyn Read) -> Result<Json, BuilderError> {
    let mut contents = Vec::new();
    match rdr.read_to_end(&mut contents) {
