rust
fn c<'a>(x: &'a u32) -> &'a u32 { // OK: used twice
    &22
}
