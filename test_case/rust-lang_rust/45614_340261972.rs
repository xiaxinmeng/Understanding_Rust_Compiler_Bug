rust
fn main() {
    struct Foo{} // warning: struct is never used
    let _ = |Foo{}| ();
}
