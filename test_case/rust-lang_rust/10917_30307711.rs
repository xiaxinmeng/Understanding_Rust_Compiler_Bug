 rust
let x: Option<(u8, i16)> = foo();
match x {
    Some(tuple @ (a, b)) => { ... }
    None => { ... }
}
