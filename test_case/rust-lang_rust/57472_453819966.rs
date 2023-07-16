Rust
pub union Punned {
    foo: [u8; 1],
    bar: [u8; 1],
}
pub fn test(punned: Punned) {
    match punned {
        Punned { foo: [_] } => println!("foo"),
        Punned { bar: [_] } => println!("bar"),
    }
}
