
error[E0275]: overflow evaluating the requirement `<T as Assoc>::B == <U as Assoc>::B`
  --> src/main.rs:13:1
   |
13 | / impl<U, T> TestTrait for Test<U, T>
14 | | where
15 | |     U: Assoc,
16 | |     T: Assoc<B = <Self as Assoc>::B>,
17 | | {
18 | | }
   | |_^
   |
note: required because of the requirements on the impl of `Assoc` for `Test<U, T>`
  --> src/main.rs:5:12
   |
5  | impl<U, T> Assoc for Test<U, T>
   |            ^^^^^     ^^^^^^^^^^

For more information about this error, try `rustc --explain E0275`.
error: could not compile `playground` due to previous error
