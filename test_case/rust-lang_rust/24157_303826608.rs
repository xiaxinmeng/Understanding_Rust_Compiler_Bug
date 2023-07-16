rust
fn is_one(a: u8) {
    match a {
        1 => true,
        _ => panic!("It's not one!"),
    }
}
