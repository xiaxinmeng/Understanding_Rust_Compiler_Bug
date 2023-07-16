rust
impl FileName {
    pub fn proc_macro_source_code(src: &str) -> FileName {
        let mut hasher = StableHasher::new();
        src.hash(&mut hasher);
        FileName::ProcMacroSourceCode(hasher.finish())
    }
}
