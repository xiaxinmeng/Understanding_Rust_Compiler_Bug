Rust
match x {
    Some(w) if guard() => ...,
    x => ...,
    Some(y) if guard2(y) => ...
    z => ...
}
