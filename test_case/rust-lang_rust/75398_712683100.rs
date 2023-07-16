
error[E0283]: type annotations needed
  --> src/main.rs:12:5
   |
6  | /     fn f<U: ?Sized>(k: &U) where
7  | |     T: std::borrow::Borrow<U> {}
   | |_____________________________- required by `S::<T>::f`
...
12 |       S::<String>::f("".as_ref());
   |       ^^^^^^^^^^^^^^ ----------- this method call resolves to `&T`
   |       |
   |       cannot infer type for type parameter `U` declared on the associated function `f`
   |
   = note: cannot satisfy `String: Borrow<_>`
