 rust
struct S;
match Some(&S) {
    Some(&S) => {},
    _x => {}
}
