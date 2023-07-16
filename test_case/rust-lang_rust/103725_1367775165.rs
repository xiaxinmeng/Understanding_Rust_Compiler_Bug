rust
let mut tmp = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
let paths = fs::read_dir(tmp).unwrap();
tmp.pop();
