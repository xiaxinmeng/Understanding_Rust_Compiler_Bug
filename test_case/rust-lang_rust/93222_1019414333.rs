rust
fn emit(...) -> Option<ErrorReported> {...}
fn emit_error(...) -> ErrorReported {
    assert!(self.level == Error);
    self.emit().unwrap()
}
