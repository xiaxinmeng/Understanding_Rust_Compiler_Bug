rust
fn provide_context<'a>(&'a self, mut request: &mut Request<'a>) {
    request
        .provide_ref::<Backtrace>(&self.backtrace)
        .provide_value::<ExitCode>(self.exit_code);
}
