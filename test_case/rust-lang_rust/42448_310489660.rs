
#![feature(decl_macro)]

macro foo($foo:ident, $bar:ident) {
  struct $foo {
    x : u8,
    y : u8
  }

  impl $foo {
    fn new (x : u8, y : u8) {
      Foo { x, y }
    }
  }

  enum $bar {
    A,
    B,
    C
  }
}

foo!{Foo,Bar}

fn main() {
  let v1 = Foo {
    x: 5, y: 4             // error: struct Foo has no fields named x,y
  };
  let v2 = Foo::new (5,4); // error: no function or associated item named new found for type Foo
  let v3 = Bar::A;         // error: no associated item named A for type Bar in this scope
}
