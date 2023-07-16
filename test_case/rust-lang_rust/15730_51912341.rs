 rust
fn main() {
    struct Foo;
    let _ = [0, ..4].as_mut_ptr() as *mut Foo;
}
