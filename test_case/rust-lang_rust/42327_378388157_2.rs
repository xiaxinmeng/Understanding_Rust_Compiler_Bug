rust
impl<E: failure::Fail> From<E> for SiteError {
    fn from(_err: E) -> Self {
        SiteError
    }
}
