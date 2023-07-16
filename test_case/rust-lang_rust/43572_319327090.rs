Rust
    fn ensure_filemap_source_present(&self, file_map: Rc<FileMap>) -> bool {
        let src = self.file_loader.read_file(Path::new(&file_map.name)).ok();
        return file_map.add_external_src(src)
    }
