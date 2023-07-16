rust
#![feature(inherent_associated_types)]

struct Foo<'a>(&'a ());

trait Trait<'a> {
    type Assoc;
}

impl<'a> Trait<'a> for Foo<'a> {
    type Assoc = ();
}

impl<'a> Foo<'a> {
    async fn foo() -> <Self as Trait<'a>>::Assoc {}
}
