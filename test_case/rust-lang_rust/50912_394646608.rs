rust
match Some(0) {
    Some(0) => {}
    Some(_) => {} // ok
    None => {}
}
