rust
let mut entries = fs::read_dir(".")?.collect::<Result<Vec<_>, io::Error>>()?;
        
entries.sort_by_key(|e| e.path());
