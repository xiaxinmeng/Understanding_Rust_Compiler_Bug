
   Compiling playground v0.0.1 (/playground)
error[E0554]: `#![feature]` may not be used on the stable release channel
 --> src/lib.rs:1:1
  |
1 | #![feature(optin_builtin_traits)]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0311]: the parameter type `S` may not live long enough
  --> src/lib.rs:32:5
   |
32 |     Box::new(async { x.await })
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: consider adding an explicit lifetime bound for `S`
   = note: the parameter type `S` must be valid for any other region...
note: ...so that the type `S` will meet its required lifetime bounds
  --> src/lib.rs:32:5
   |
32 |     Box::new(async { x.await })
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0554`.
error: could not compile `playground`.

To learn more, run the command again with --verbose.

