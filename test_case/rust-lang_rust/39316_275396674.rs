rust
fn test() {
    let mut v = 0u8;
    let r = func(&mut v);
    println!("{}", v);
}

fn func(v: &mut u8) -> &u8 {
    v
}
