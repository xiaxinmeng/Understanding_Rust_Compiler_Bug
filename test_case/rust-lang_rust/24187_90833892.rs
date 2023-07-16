 rust
fn main() {
    let ref mut x = Box::new(42);
    let ref y = x;
    **x = 0; // error: cannot assign to `**x` because it is borrowed
}
