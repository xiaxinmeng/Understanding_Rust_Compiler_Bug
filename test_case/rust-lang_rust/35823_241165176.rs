 rust
let mut file = File::open(entry.path())?;
let mut file_contents = String::new();
file.read_to_string(&mut file_contents).unwrap();
