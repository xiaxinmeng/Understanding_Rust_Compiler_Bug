 Rust
fn nullptr<T: ?Sized>() -> *const T {
    0 as *const _
}
fn main() {
    nullptr::<[();3]>();
    nullptr::<[()]>();
}
