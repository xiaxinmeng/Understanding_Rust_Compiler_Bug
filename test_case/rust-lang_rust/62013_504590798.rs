rust
pub struct Demo {
    foo: Option<Box<Demo>>,
}

pub fn foo1(mut a: &mut Demo) {
    if let Some(ref mut b) = *&mut a.foo {
        a = b;
    }
    a.foo = None;
}
