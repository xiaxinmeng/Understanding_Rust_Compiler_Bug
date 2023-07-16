rust
match a {
  Some(b @ MyEnum::Foo(..)) => {
    let c = match b {
      &MyEnum::Foo(ref c) => c,
      _ => unreachable!(),
    };
  }
}
