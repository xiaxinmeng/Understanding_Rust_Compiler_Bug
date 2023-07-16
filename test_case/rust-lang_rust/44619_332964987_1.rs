Rust
is_handler((|x: &str| x) as for<'r> fn(&'r str) -> &'r str);
