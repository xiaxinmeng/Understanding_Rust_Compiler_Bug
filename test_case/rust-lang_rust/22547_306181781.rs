
error[E0585]: found a documentation comment that doesn't document anything
  --> src/lib.rs:10:17
   |
10 |     Some(T),    /// Some value `T`
   |                 ^^^^^^^^^^^^^^^^^^
   |
   |
   = help: doc comments must come before what they document, maybe a comment
     was intended with `//`?
