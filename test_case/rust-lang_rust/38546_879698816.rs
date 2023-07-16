rust
impl SourceFile {
    /// Load a file relative to this source file and parse it as a `TokenStream`.
    ///
    /// The file path will be included in the resulting dep-info and will trigger recompilation when it changes.
    pub fn include(&self, path: impl AsRef<Path>) -> Result<TokenStream, IncludeError> { ... }

    /// Load a file relative to this source file as a UTF-8 string.
    ///
    /// The file path will be included in the resulting dep-info and will trigger recompilation when it changes.
    pub fn include_str(path: impl AsRef<Path>) -> Result<String, IncludeError> { ... }
}
