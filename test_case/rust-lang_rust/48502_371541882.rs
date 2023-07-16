rust
macro_rules! foo {
  ($t:ty) => {
    impl $t for ... { }
  }
}
