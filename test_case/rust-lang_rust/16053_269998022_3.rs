rust
match a {
  Some(&MyEnum::Foo(ref c)) => {
    let b = MyEnum::Foo(c);
    // mismatched types
    // expected type `Bar`
    //    found type `&Bar`
  }
}
