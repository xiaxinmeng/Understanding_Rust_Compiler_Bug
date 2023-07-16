rust
$ cat src/lib.rs 
//! Check out the docs for [`SpecialFoo::foo`].

pub trait Foo {
   /// This function generally does foo without side effects.
   /// See [`SpecialFoo::foo`] for an example.
   fn foo();
}

pub struct SpecialFoo {}

impl Foo for SpecialFoo {
    /// This is a good example of doing [`Foo::foo`]
    fn foo() {}
}
