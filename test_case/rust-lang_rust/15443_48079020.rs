 rust
pub type LoadError = LoadError::LoadError;
pub mod LoadError {
    pub enum LoadError {
        ErrSyntax,
        ErrMem
    }
}
