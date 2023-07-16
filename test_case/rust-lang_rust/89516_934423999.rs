rust
compiler/rustc_data_structures/src/memmap.rs
    pub unsafe fn map(mut file: File) -> io::Result<Self> {
---
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;

src/tools/rls/rls-vfs/src/lib.rs
        let mut file = match fs::File::open(file_name) {
--
        let mut buf = vec![];
        if file.read_to_end(&mut buf).is_err() {

src/tools/cargo/crates/cargo-util/src/paths.rs
        let mut f = OpenOptions::new()
--
            .open(&path)?;
        let mut orig = Vec::new();
        f.read_to_end(&mut orig)?;
