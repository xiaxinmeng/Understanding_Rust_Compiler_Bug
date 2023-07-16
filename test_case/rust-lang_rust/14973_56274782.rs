 rust
fn f(v: Vec<uint>) {
    for x in v.iter() {
        let y: uint = if x > 0 {
            break;
        } else {
            42u
        };
        println!("{}", y);
    }
}
