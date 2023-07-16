rust
pub struct Demo {
    foo: Option<Box<Demo>>,
}

pub fn foo1<'a>(mut a: &'a mut Demo) {
    if let Some(ref mut b) = *&mut a.foo {
        // This assignment is what causes `a.foo` to be borrowed for `'a`
        // because that is what the type of `a` mandates: `&'a mut Demo`
        a = b;
    }
    a.foo = None;
}
