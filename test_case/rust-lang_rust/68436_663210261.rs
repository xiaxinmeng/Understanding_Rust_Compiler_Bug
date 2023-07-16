rust
#![feature(const_generics)]
#![allow(const_err)]

fn foo<const C: u8>() {
  if C > 0 {
    println!("Foo gives {}", 25 / C);
    // need `allow(const_err)` here because of #74696, this is not a restriction of const generics
  }
  else {
    println!("Foo gives 0");
  }
}

fn main() {
  foo::<0>();
}
