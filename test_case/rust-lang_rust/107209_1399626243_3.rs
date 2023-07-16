rust
error[E0284]: type annotations needed
  --> src/main.rs:28:1
   |
28 | impl<T> ExampleTrait for Sad<T>
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type
   |
   = note: cannot satisfy `<[T; 10] as ExampleTrait>::AssociatedType == _`

error[E0284]: type annotations needed
  --> src/main.rs:28:1
   |
28 | / impl<T> ExampleTrait for Sad<T>
29 | | where
30 | |     [T; MYSIZE]: ExampleTrait,
31 | | <[T; MYSIZE] as ExampleTrait>::AssociatedType: Clone,
...  |
35 | |     type AssociatedType = T;
36 | | }
   | |_^ cannot infer type
   |
   = note: cannot satisfy `<[T; 10] as ExampleTrait>::AssociatedType == _`

error[E0284]: type annotations needed
  --> src/main.rs:35:5
   |
35 |     type AssociatedType = T;
   |     ^^^^^^^^^^^^^^^^^^^ cannot infer type
   |
   = note: cannot satisfy `<[T; 10] as ExampleTrait>::AssociatedType == _`

For more information about this error, try `rustc --explain E0284`.
