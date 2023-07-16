rust
fn test_deconstruct_move() {
    let mut maybe_s = Some(SomeStruct { val: 0 });
    if let Some(s) = maybe_s {
        maybe_s = Some(s);
    }
    println!("{:?}", maybe_s);
}
