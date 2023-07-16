rust
fn static_handler(_: &str) -> &'static str { "hi" }

is_handler(static_handler);

is_handler(|_| "hi");

is_handler(|x| x);
