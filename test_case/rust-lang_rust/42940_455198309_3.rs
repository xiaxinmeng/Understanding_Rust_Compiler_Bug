rust
fn login<'a>(client: &'a Client, username: &str) -> impl Future + 'a { .. }
