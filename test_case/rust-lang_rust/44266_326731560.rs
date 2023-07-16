rust
match 0u8 {
    x if x == Thing::Foo as u8 => { ... }
    x if x == Thing::Bar as u8 => { ... }
    _ => println!("..."),
}
