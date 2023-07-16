rust
    pub fn lookup_source_file_idx(&self, pos: BytePos) -> usize {
        self.files.borrow().source_files.binary_search_by_key(&pos, |key| key.start_pos)
            .unwrap_or_else(|p| p - 1)
    }
