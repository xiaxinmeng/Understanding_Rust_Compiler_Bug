
    pub fn file_suffix(&self) -> Option<&OsStr> {
        self.file_name().map(split_file_at_dot).and_then(|(_before, after)| Some(after))
    }
