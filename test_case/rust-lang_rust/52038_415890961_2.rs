rust
// 2015
macro_rules! foo {
  () => {
    fn bar() {
      use ::x::y; // can this resolve to `crate::x::y` when invoked in a 2018 crate?
    }
  }
}
