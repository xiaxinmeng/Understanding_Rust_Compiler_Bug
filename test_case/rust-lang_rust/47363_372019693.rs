rust
    // Pseudo-code. OsStr::trim_right does not exist, implement it yourself.
    #[cfg(windows)]
    pub fn clean_file_name(f: &OsStr) -> &OsStr {
        f.trim_right(&['.', ' '])
    }
    #[cfg(not(windows))]
    pub fn clean_file_name(f: &OsStr) -> &OsStr {
        f
    }
    