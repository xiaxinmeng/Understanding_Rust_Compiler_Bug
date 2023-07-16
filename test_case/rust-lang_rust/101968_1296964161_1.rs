rust
fn future_closure_arg(arg: impl Fn() -> impl Future<Output = bool>) {}
// =>
fn future_closure_arg<Fut: Future<Output = bool>>(arg: impl Fn() -> Fut) {}
