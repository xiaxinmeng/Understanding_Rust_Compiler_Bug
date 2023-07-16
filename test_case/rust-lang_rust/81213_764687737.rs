plain
    Checking strip-ansi-escapes v0.1.0
    Checking aho-corasick v0.7.15
    Checking bstr v0.2.14
   Compiling quote v1.0.8
error[E0277]: the trait bound `ScopedJoinHandle<'_, T>: std::sealed::Sealed` is not satisfied
    |
    |
552 |         impl<T> JoinHandleExt for ScopedJoinHandle<'_, T> {
    |                 ^^^^^^^^^^^^^ the trait `std::sealed::Sealed` is not implemented for `ScopedJoinHandle<'_, T>`
   ::: /checkout/library/std/src/sys/unix/ext/thread.rs:17:26
    |
    |
17  | pub trait JoinHandleExt: Sealed {
    |                          ------ required by this bound in `JoinHandleExt`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
error: could not compile `crossbeam-utils`
