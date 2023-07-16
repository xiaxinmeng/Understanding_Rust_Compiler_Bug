 rust
struct S(i32, i32);
fn t(s: S) -> i64 {
    assert!(size_of<S>() == size_of<i64>());
    transmute(s)
}
