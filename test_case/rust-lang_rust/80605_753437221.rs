rust
impl Error for ProcessError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ProcessError::ParseFieldError { source, .. } => source.as_ref().map(|e| e as _),
            /* ... */
        }
    }
}
