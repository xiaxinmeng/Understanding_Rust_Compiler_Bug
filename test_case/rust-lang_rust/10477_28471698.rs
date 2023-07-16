 rust
  struct Foo;
  impl Foo { fn foo(&self) { bar() } }
  fn bar() {
     fn baz() {}
  
     Foo.foo();
     baz();
  }
  
  fn main() {}
  