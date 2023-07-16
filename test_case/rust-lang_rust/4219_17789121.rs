 rust
macro_rules! a(
  () => (pub mod a {
    #[cfg(a)] pub fn foo() {}
    #[cfg(b)] pub fn foo() {}
  })
)

a!()

fn main() { a::foo(); }
