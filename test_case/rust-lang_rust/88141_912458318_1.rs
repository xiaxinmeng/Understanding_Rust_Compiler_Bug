
error[E1337]: `T` does not satisfy `Unsize<dyn Trait>`
 --> src/main.rs:4:5
  |
3 | fn convert<T: Trait + ?Sized>(b: Box<T>) -> Box<dyn Trait> {
  |            - this type parameter needs to be `Unsize<dyn Trait>`
4 |     b
  |     ^ isn't known to satisfy `Unsize<dyn Trait>` at compile-time
  |
  = note: only types that satisfy `Unsize<dyn Trait>` can be cast to the object type `dyn Trait`
  = hint: add an `Unsize<dyn Trait>` bound:
...
