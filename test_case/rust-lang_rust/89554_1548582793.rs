rust
#![deny(non_exhaustive_omitted_patterns)]
match (&x, true) {
    (NonExhaustiveEnum::A, true) => {}
    _ => {} // detected
}
match (true, &x) {
    (true, NonExhaustiveEnum::A) => {}
    _ => {} // not detected
}
