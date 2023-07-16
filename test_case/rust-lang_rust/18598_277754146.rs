rust
pub struct Foo {
    flag: bool,
    data: str,
}

#[no_mangle]
pub extern fn drop(_b: Box<Foo>) {
}
