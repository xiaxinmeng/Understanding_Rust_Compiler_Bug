
error[E0512]: transmute called with two types that may have different sizes
 --> test.rs:8:5
  |
8 |     ::std::mem::transmute(x)
  |     ^^^^^^^^^^^^^^^^^^^^^ source and target type of transmute may have different sizes
  |
  | note: source type is `<C as TypeConstructor<'a>>::T` and target type is `<C as TypeConstructor<'b>>::T`
          their size can vary because of `<C as TypeConstructor>::T`
