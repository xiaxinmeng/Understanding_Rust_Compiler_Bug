rust
    #[cfg(windows)]
    fn clean_reserved_path(filename: &OsStr) -> Option<PathBuf> {
        if let Some(capture) = filename.capture(r"^(COM[0-9¹²³]|…)(?:[:. ]*[:.].*)?$") {
           Some(r"\\.\" + capture.get(1))
        } else {
           None
        }
    }
    #[cfg(not(windows))]
    fn clean_reserved_path(_: &OsStr) -> Option<PathBuf> { 
        None
    }
    