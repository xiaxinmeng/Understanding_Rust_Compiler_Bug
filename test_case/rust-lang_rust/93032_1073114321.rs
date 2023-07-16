rust
let x: Option<T> = Some(_);
match x {
    Some(_) /* must be _ */ => true,
    None => false,
}
