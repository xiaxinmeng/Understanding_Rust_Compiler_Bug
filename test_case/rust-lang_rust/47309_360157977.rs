
#![feature(generic_associated_types)]

pub trait Foo<'a> {}
pub trait Bar {
  type Foo<'a> : Foo<'a>;    
  fn foo<'a>(&'a self) -> Self::Foo;
}

fn main() {}
