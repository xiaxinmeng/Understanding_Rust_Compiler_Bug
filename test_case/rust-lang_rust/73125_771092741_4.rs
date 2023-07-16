
impl ExitStatusError {
    /// The returned `io::Error` will have kind `Other` and have
    /// this `ExitStatusError` as its inner error.  This is a convenience
    /// wrapper equivalent to calling `io::Error::new`.
    /// 
    /// Provided for situations where it is not convenient to deal
    /// with a subprocess which ran but failed differently to other
    /// errors which occur when running a subprocess.
    fn into_io_error(self) -> std::io::Error { ... }
}
