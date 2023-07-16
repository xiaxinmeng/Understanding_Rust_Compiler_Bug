rust
impl<E: StdError + Send + Sync + 'static> Fail for E {}
