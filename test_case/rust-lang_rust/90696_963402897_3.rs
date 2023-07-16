
warning: the feature `generic_associated_types` is incomplete and may cause the compiler to crash
 --> src/main.rs:1:12
  |
1 | #![feature(generic_associated_types)]
  |            ^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(incomplete_features)]` on by default

error[E0109]: lifetime arguments are not allowed for this type
  --> src/main.rs:18:43
   |
18 |     let result: PhantomData<S::Associated<'a>> = PhantomData;
   |                                           ^^ lifetime argument not allowed

error[E0109]: lifetime arguments are not allowed for this type
  --> src/main.rs:24:19
   |
24 |     S::Associated<'a>: Send,
   |                   ^^ lifetime argument not allowed

error: aborting due to 2 previous errors
