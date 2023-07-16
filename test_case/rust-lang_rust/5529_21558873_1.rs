 rust
  extern mod foo;
  
  fn main() {
      let t = foo::two();
      println(fmt!("t:%?", t));
  }
  