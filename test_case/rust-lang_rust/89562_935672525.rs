rust
fn is_send<T: Send>(_x: T) {}

// error: future cannot be sent between threads safely
fn main() { is_send(foo()); }
