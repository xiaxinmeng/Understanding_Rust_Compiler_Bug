rust
impl std::error::Error for Error {
    fn provide<'a>(&'a self, mut req: Requisition<'a, '_>) {
        req
            .provide_ref::<MyBacktrace>(&self.backtrace)
            .provide_ref::<dyn std::error::Error + 'static>(&self.source);
    }
}
