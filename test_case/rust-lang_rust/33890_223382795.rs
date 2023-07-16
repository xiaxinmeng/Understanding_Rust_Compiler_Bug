 rust
fn exists<P: AsRef<Path>>(path: P) -> io::Result<bool> {
    match fs::metadata(path) {
        Ok(_) => Ok(true),
        Err(ref err) if err.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(err) => Err(err)
    }
}
