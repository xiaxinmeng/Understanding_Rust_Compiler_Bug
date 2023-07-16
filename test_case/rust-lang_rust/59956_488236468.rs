console
$ cat bug.rs
struct Bug<B = Self>;

impl Bug {}
$ rustc +1.31.1 bug.rs
error[E0411]: cannot find type `Self` in this scope
 --> bug.rs:1:16
  |
1 | struct Bug<B = Self>;
  |                ^^^^ `Self` is only available in traits and impls

error[E0601]: `main` function not found in crate `bug`
  |
  = note: consider adding a `main` function to `bug.rs`

error[E0392]: parameter `B` is never used
 --> bug.rs:1:12
  |
1 | struct Bug<B = Self>;
  |            ^ unused type parameter
  |
  = help: consider removing `B` or using a marker such as `std::marker::PhantomData`

error: aborting due to 3 previous errors

Some errors occurred: E0392, E0411, E0601.
For more information about an error, try `rustc --explain E0392`.
$ rustc +1.32.0 bug.rs
error[E0601]: `main` function not found in crate `bug`
  |
  = note: consider adding a `main` function to `bug.rs`

error: internal compiler error: src/librustc/ty/subst.rs:491: Type parameter `B/#0` (B/0) out of range when substituting (root type=Some(Bug<B>)) substs=[]
 --> bug.rs:3:6
  |
3 | impl Bug {}
  |      ^^^

thread 'main' panicked at 'Box<Any>', src/librustc_errors/lib.rs:538:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0601`.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.32.0 (9fda7c223 2019-01-16) running on x86_64-unknown-linux-gnu
