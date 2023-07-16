
error[[E0284]](https://doc.rust-lang.org/stable/error-index.html#E0284): type annotations needed
  --> src/lib.rs:17:23
   |
17 |     pub fn MOVES_SELF(self) {}
   |                       ^^^^ cannot infer type
   |
   = note: cannot satisfy `<[T] as ToOwned>::Owned == _`
note: required because it appears within the type `Bar<'_, T>`
  --> src/lib.rs:3:12
   |
3  | pub struct Bar<'a, T>
   |            ^^^
help: function arguments must have a statically known size, borrowed types always have a known size
   |
17 |     pub fn MOVES_SELF(&self) {}
   |                       +
