rust
match a {
  Some(b @ &MyEnum::Foo(ref c)) => {
  }
}
