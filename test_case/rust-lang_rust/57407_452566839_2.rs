rust
  macro_rules! extern_something {
      ($alias:ident) => { extern crate $alias as the_alias; }
  }
 
  extern_something!(self);
 
  fn main() {
      assert_eq!(the_alias::the_answer(), 42);
  }
  