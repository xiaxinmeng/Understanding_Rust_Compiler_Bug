rust
fn hello_shim<'s>(v: &Bar<'s>) {
    v.hello();
}
