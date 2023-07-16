
warning: conflicting implementations of trait `T` for type `fn(&_)`:
 --> src/main.rs:6:1
  |
5 | impl<A> T for fn(A) {}
  | ------------------- first implementation here
6 | impl<A> T for fn(&A) {} // cause #[warn(coherence_leak_check)] on nightly 2020-03-29
  | ^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `fn(&_)`
  |
  = note: `#[warn(coherence_leak_check)]` on by default
  = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
  = note: for more information, see issue #56105 <https://github.com/rust-lang/rust/issues/56105>
  = note: this behavior recently changed as a result of a bug fix; see rust-lang/rust#56105 for details
