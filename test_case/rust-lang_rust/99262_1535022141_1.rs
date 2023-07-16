rust
impl From<io::Error> for BinstallError {
    fn from(err: io::Error) -> Self {
        if err.get_ref().is_some() {
            let kind = err.kind();

            let inner = err
                .into_inner()
                .expect("err.get_ref() returns Some, so err.into_inner() should also return Some");

            inner
                .downcast()
                .map(|b| *b)
                .unwrap_or_else(|err| BinstallError::Io(io::Error::new(kind, err)))
        } else {
            BinstallError::Io(err)
        }
    }
}
