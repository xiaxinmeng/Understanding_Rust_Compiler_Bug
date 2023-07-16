
error: implementation of `Send` is not general enough
  --> src/main.rs:26:5
   |
26 |     future::<'a, S, _>(f::<'a, S>());
   |     ^^^^^^^^^^^^^^^^^^ implementation of `Send` is not general enough
   |
   = note: `<S as Trait>::Associated<'0>` must implement `Send`, for any lifetime `'0`...
   = note: ...but `Send` is actually implemented for the type `<S as Trait>::Associated<'a>`
