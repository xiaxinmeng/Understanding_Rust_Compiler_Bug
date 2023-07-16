rust
  macro_rules! alias_self {
      ($alias:ident) => { extern crate self as $alias; }
  }
  
  alias_self!(the_alias);
  
  fn main() {
      assert_eq!(the_alias::the_answer(), 42);
  }
  