 rust
let x: Result<u8, Void> = Ok(1);
match x {
    Ok(b) => (),
    Err(_) => () 
}
