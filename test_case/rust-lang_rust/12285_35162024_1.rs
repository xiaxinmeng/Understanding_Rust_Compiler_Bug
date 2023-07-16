 rust
struct S;
match Some(&S) {
    Some(&S) => {},
    None => {}
}
