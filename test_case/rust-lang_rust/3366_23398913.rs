 rust
fn needs_send<T: Send>(_: T) {}
fn no_send<SomeParamName>(x: SomeParamName) { needs_send(x) }
