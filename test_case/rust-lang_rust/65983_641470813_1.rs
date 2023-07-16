rust
// crate a

pub mod bar {
   pub struct Bar;
}

pub mod foo {
  use crate::bar;
  /// link to [bar::Bar]
  pub struct Foo;
}

// crate b

pub use a::foo::Foo;
