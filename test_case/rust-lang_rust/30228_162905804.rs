 rust
pub fn foo(a: &mut Box<i32>, b: Box<i32>) {
    *a = b;
}
