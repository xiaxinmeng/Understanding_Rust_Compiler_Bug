Rust
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() {
    let path = Path::new(".");
    sorted_read_dir(&path);
}

fn sorted_read_dir(path: &Path) -> io::Result<()> {
    let dir = fs::read_dir(path)?;
    let mut entries: Vec<PathBuf> = dir
        .filter(Result::is_ok)
        .map(|e| e.unwrap().path())
        .collect();
    entries.sort();
    for entry in entries {
        println!("{:?}", entry);
    }
    Ok(())
}
