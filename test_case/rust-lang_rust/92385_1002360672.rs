plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0493]: destructors cannot be evaluated at compile-time
   --> library/core/src/result.rs:647:21
    |
647 |     pub const fn ok(self) -> Option<T> {
    |                     ^^^^ constant functions cannot evaluate destructors
error[E0493]: destructors cannot be evaluated at compile-time
   --> library/core/src/result.rs:673:22
    |
    |
673 |     pub const fn err(self) -> Option<E> {
    |                      ^^^^ constant functions cannot evaluate destructors
For more information about this error, try `rustc --explain E0493`.
error: could not compile `core` due to 2 previous errors
Build completed unsuccessfully in 0:01:08
