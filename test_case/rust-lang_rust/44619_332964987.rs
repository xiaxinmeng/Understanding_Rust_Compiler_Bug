Rust
is_handler(|_| "hi"); //~ ERROR
is_handler(mk_handler(|_| "hi")); // ok
