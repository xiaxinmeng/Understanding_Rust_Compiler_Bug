rust
fn foo(x: bool) -> &'static str {
    match x as u8 {
        0 => "null",
        1 => "one",
        2 => "meh",
        _ => "muh",
    }
}
