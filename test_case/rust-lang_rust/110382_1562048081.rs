rust
pub enum MyError {
  ExternalBarError(BarErrorStructure),
  ExternalFooError(FooErrorStructure),
  MyCustomErrorVariant,
  ...
}

fn do_some_testing(error: &MyError) {
  // It is not possible to use `assert_eq!` because third-parties don't implement `PartialEq`
  assert!(matches!(error, MyError::MyCustomErrorVariant));
}
