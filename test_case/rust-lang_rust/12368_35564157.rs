 rust
fn drop(&mut self) {
    if !task::failing() {
        match *self {
            Err(err) => fail!("unhandled IO error: {}" ,err),
            Ok(_) => {}
        }
    }
}
