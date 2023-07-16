rust
fn foo() -> impl Fn() -> impl Debug + Send { }

// Is this `-> impl Fn() -> (impl Debug) + Send` or `impl Fn() -> (impl Debug + Send)` ?
