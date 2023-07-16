 rust
...
fn finished(self_: &mut StateMachineIter) -> Option<(&'static str)> {
    self_.statefn = &finished;
    return None;
}
...
