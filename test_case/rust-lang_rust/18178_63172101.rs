 rust
#![feature(associated_types)]

pub trait Cancel {
    fn cancel(self);
}

pub trait Foo {
    type C: Cancel;

    fn stuff(self) -> <Self as Foo>::C;
    fn trait_doit(self) { // this method typechecks
        let cancel_me = self.stuff();
        cancel_me.cancel();
    }
}

fn doit<F: Foo>(f: F) { // this function doesn't typecheck
    let cancel_me = f.stuff();
    cancel_me.cancel();
}

pub fn main() {
}
