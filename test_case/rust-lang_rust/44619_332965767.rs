rust
fn mk_handler<F: Fn(&str) -> &str>(f: F) -> F { f }

is_handler(mk_handler(|_| "hi"));

is_handler(mk_handler(|x| x));
