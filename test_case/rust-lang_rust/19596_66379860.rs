 rust
fn method_ok_unboxed<F>(&self, f: F) -> bool where F: Fn(()) -> bool {
    self.method_ok_unboxed(f);
}
