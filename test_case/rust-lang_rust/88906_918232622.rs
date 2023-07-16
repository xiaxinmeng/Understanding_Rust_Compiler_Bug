plain
    Checking rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
error: variable does not need to be mutable
  --> compiler/rustc_data_structures/src/functor.rs:27:17
   |
27 |             let mut raw: Box<mem::MaybeUninit<T>> = Box::from_raw(raw.cast());
   |                 |
   |                 help: remove this `mut`
   |
   |
   = note: `-D unused-mut` implied by `-D warnings`
error[E0382]: use of moved value: `raw`
  --> compiler/rustc_data_structures/src/functor.rs:31:13
   |
   |
27 |             let mut raw: Box<mem::MaybeUninit<T>> = Box::from_raw(raw.cast());
   |                 ------- move occurs because `raw` has type `Box<MaybeUninit<T>>`, which does not implement the `Copy` trait
28 |             // SAFETY: Write the mapped value back into the `Box`.
29 |             raw.write(f(value));
   |             --- value moved here
30 |             // SAFETY: We just initialized `raw`.
31 |             raw.assume_init()
   |             ^^^ value used here after move
For more information about this error, try `rustc --explain E0382`.
error: could not compile `rustc_data_structures` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
