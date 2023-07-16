rust
match f {
  Foo::A => {
    match g {
      1 => { return 5; }
      2 => { return 10; }
      _ => {}
    }
  }
  Foo::B => {
    match g {
      2 => { return 15; }
      _ => {}
    }
  }
}
