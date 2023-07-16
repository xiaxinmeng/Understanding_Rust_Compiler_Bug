rust
pub trait Foo {
    /// Do something
    fn do_something(&self);

    /// Do something twice
    fn do_something_twice(&self) {
        self.do_something(); self.do_something();
    }
}

pub struct Baz;

impl Foo for Baz {
    /// Do something special since this is a Baz instance and bla bla…
    fn do_something(&self) {
        println!("Foo");
    }
}
