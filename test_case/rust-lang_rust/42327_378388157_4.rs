rust
impl<E: std::error::Error> From<E> for SiteError {
    fn from(_err: E) -> Self {
        SiteError
    }
}
