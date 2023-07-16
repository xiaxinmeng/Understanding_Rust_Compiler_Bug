rust
    #[cfg(unix)]
    fn write_path<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        use std::os::unix::ffi::OsStrExt;
        let path = path.as_ref().as_os_str().as_bytes();
        self.wtr.write_all(path)
    }

    #[cfg(not(unix))]
    fn write_path<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref().to_string_lossy();
        self.wtr.write_all(path.as_bytes())
    }
