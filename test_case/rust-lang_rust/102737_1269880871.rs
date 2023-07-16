plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: variable does not need to be mutable
  --> library/core/src/future/poll_fn.rs:62:13
   |
62 |     fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
   |             |
   |             help: remove this `mut`
   |
   |
   = note: `-D unused-mut` implied by `-D warnings`

error[E0596]: cannot borrow data in dereference of `Pin<&mut PollFn<F>>` as mutable
   |
   |
63 |         (&mut self.f)(cx)
   |         ^^^^^^^^^^^^^ cannot borrow as mutable
   |
   = help: trait `DerefMut` is required to modify through a dereference, but it is not implemented for `Pin<&mut PollFn<F>>`
For more information about this error, try `rustc --explain E0596`.
error: could not compile `core` due to 2 previous errors
Build completed unsuccessfully in 0:01:37
