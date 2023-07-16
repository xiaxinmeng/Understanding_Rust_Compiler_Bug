rust
fn test() {
    let mut v = 0u8;
    let r: &u8;
    { r = &mut v; }
    println!("{}", v);
}
