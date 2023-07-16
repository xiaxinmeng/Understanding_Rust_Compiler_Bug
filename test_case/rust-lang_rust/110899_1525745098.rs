rs
fn wrap(self: Wrap<{ fn bar(&self) {} }>) -> &() {
  &()
}
