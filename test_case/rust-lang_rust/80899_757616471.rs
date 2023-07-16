rust
fn transmute_to_self<T: Sized>(v: [T; 10]) -> impl Sized {
    v
}
