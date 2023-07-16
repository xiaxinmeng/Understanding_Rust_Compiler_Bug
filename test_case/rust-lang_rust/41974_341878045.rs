
error[E0119]: conflicting implementations of trait `std::ops::Drop` for type `std::boxed::Box<_>`:
  --> src/stream.rs:76:1
   |
76 | / impl<T> Drop for T where T: Stream {
77 | |     fn drop(&mut self) {
78 | |         unsafe {
79 | |             nn_close(self.as_raw_fd());
80 | |         }
81 | |     }
82 | | }
   | |_^
   |
   = note: conflicting implementation in crate `alloc`

error[E0120]: the Drop trait may only be implemented on structures
  --> src/stream.rs:76:18
   |
76 | impl<T> Drop for T where T: Stream {
   |                  ^ implementing Drop requires a struct

error: internal compiler error: /checkout/src/librustc_typeck/check/dropck.rs:64: should have been rejected by coherence check: T
  --> src/stream.rs:76:1
   |
76 | / impl<T> Drop for T where T: Stream {
77 | |     fn drop(&mut self) {
78 | |         unsafe {
79 | |             nn_close(self.as_raw_fd());
80 | |         }
81 | |     }
82 | | }
   | |_^

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.21.0 (3b72af97e 2017-10-09) running on x86_64-unknown-linux-gnu

thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc_errors/lib.rs:437:8
