
error: recursion limit reached while expanding the macro `match_ignore_ascii_case`
  --> src/main.rs:3:27
   |
3  |     ( $($rest:tt)* ) => { match_ignore_ascii_case!(@inner $($rest)*) };
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
12 |     match_ignore_ascii_case!(2 => 3);
   |     --------------------------------- in this macro invocation
   |
   = help: consider adding a `#![recursion_limit="128"]` attribute to your crate
