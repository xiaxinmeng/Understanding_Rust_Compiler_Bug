rust
  macro_rules! accept_item { ($x:item) => {} }

  accept_item! {
      extern crate self;
  }
  