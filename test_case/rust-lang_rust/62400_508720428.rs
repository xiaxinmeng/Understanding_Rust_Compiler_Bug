rust
fn foo<'a>(x: &'a mut [&'a mut &'static str]) { x[0] = x[1]; }
