
$ cargo test -- --nocapture
   Compiling params-attribute-example v0.1.0 (/home/centril/programming/rust/params-attribute-example)
send_help fn hello(#[angery(true)] a: i32, b: i32) { }
error[E0658]: the attribute `angery` is currently unknown to the compiler and may have meaning added to it in the future
