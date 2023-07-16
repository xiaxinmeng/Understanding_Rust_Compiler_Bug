 rust
pub trait Foo { }

static mut _foo: Option<&'static mut Foo> = None;

pub fn get_foo() -> &'static mut Foo {
    unsafe { match _foo {
            Some(ref mut x) => &mut **x,
            None => panic!(),
    } }
}

fn main() {}
