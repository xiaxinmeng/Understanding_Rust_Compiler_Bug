rust
pub mod a {    // outer decl
  pub mod a {} // inner decl
}
pub mod b { 
  use super::*; // outer use
  pub fn f() {     // inner use
    use a::*;
  }
}
