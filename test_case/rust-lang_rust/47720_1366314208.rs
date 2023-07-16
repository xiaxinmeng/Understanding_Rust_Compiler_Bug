
error[E0275]: overflow evaluating the requirement `for<'a> &'a _: TextSized`
  --> src/lib.rs:28:5
   |
28 |     assert_TextSized::<&()>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`playground`)
note: required for `&'a _` to implement `for<'a> TextSized`
  --> src/lib.rs:19:16
   |
19 | impl<D: Deref> TextSized for &'_ D
   |                ^^^^^^^^^     ^^^^^
   = note: 126 redundant requirements hidden
   = note: required for `&()` to implement `TextSized`
note: required by a bound in `assert_TextSized`
  --> src/lib.rs:5:24
   |
5  | fn assert_TextSized<T: TextSized>() {}
   |                        ^^^^^^^^^ required by this bound in `assert_TextSized`
