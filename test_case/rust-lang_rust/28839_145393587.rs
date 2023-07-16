 rust
use std::ptr;
pub trait Foo { }
static mut _foo: Option<&'static mut Foo> = None;
pub fn get_foo() -> &'static mut Foo {
    unsafe {
        match ptr::read(&_foo) {
            Some(x) => x,
            None => panic!(),
        }
    }
}
