
error[E0308]: mismatched types
  --> src/lib.rs:4:11
   |
4  | fn f() -> impl Future<Output = ()> + Send {
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
...
8  |             stream::empty().for_each(|_: ()| async { drop(a) }),
   |                                                    -----------
   |                                                    |
   |                                                    the expected generator
   |                                                    the found generator
   |
   = note: expected opaque type `impl futures::Future`
              found opaque type `impl futures::Future`
