
fn cause(&self) -> Option<&dyn Error> {
    self.cause.as_ref().map(|e| -> &dyn Error { e })
}
