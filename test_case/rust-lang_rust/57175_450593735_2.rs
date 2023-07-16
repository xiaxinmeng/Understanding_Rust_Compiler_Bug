rust
  const fn bar() -> [u32; 500] {
      let mut foo = [0; 500];
      foo[3] = 42;
      foo
  }
  