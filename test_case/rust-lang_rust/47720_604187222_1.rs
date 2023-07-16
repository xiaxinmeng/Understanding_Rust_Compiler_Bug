
error[E0275]: overflow evaluating the requirement `for<'a> &'a _: TextSized`
  --> src/lib.rs:28:5
   |
5  | fn assert_TextSized<T: TextSized>() {}
   |    ----------------    --------- required by this bound in `assert_TextSized`
...
28 |     assert_TextSized::<&()>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`playground`)
   = note: required because of the requirements on the impl of `for<'a> TextSized` for `&'a _`
     [snip about 123 copies of that line]
   = note: required because of the requirements on the impl of `for<'a> TextSized` for `&'a _`
   = note: required because of the requirements on the impl of `TextSized` for `&()`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0275`.
error: could not compile `playground`.
