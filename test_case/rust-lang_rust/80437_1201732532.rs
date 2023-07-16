rust
#![feature(box_into_inner)]

pub trait Foo {
    // ... other stuff ...

    fn finish(self);
}

impl<T: Foo> Foo for Box<T> {
    // ... other stuff ...

    fn finish(self) {
        Box::into_inner(self).finish()
    }
}
