
error[E0275]: overflow evaluating the requirement `MyTuple<B, U>: Shel<MyTuple<_, _>>`
 --> q_105269.rs:7:1
  |
7 | impl<U, B, Ur, Br> Shel<MyTuple<Ur, Br>> for MyTuple<U, B>
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: consider increasing the recursion limit by adding a `#![recursion_limit = "80"]` attribute to your crate (`q_105269`)
note: required for `MyTuple<U, B>` to implement `Shel<MyTuple<Ur, Br>>`
 --> q_105269.rs:7:20
  |
7 | impl<U, B, Ur, Br> Shel<MyTuple<Ur, Br>> for MyTuple<U, B>
  |                    ^^^^^^^^^^^^^^^^^^^^^     ^^^^^^^^^^^^^
