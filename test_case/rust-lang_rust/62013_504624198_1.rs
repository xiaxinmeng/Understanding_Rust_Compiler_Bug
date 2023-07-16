rust
pub struct Demo {
    foo: Option<Box<Demo>>,
}

pub fn foo1<'a>(mut a: &'a mut Demo) {
    /* 'b: */ {
        // Just borrowing for `'b`. Nothing forces a borrow for `'a`.
        let _ = &mut a.foo;
    }
    a.foo = None;
}
