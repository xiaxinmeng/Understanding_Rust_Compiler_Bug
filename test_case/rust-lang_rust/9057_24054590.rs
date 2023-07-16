 rust
#[deprecated]
mod x {
  #[experimental]
  mod y {
    fn foo() {
      experimental(); // ok
      deprecated(); // "warning: use of deprecated item"
    }
  }
}
