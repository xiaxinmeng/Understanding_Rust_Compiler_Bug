 rust
    fn f(&self, path: &Path) -> FileMeta {
        let m = FileMeta;
        let p = PathBuf::new(path);
        match self.h2.get(&p) {
            Some(meta) => m,
            None => m
        }
    }
