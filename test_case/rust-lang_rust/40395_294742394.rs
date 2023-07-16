rust
#[derive(Default)]
pub struct Foo;

pub type AliasedFoo = Foo;

pub trait Bar {
    fn yep(&self) {}
}

impl Bar for Foo {}

pub trait Baz {
    fn nope(&self) {}
}

impl Baz for AliasedFoo {}

#[cfg(test)]
mod tests {
    use {Foo, AliasedFoo, Bar, Baz};
    #[test]
    fn it_works() {
        Foo::default().yep();
        Foo::default().nope();
        AliasedFoo::default().yep();
        AliasedFoo::default().nope();
    }
}
