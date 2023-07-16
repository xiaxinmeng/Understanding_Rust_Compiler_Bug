
   Compiling hello_world v0.1.0 (/Users/sumukhgovindaraju/github.com/Penn-Electric-Racing/hello_world)
error[E0433]: failed to resolve: could not find `__OsLocalKeyInner` in `thread`
 --> src/lib.rs:3:1
  |
3 | thread_local!(static LOCK_HELD: Cell<bool> = Cell::new(false));
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ could not find `__OsLocalKeyInner` in `thread`
  |
  = note: this error originates in the macro `$crate::__thread_local_inner` which comes from the expansion of the macro `thread_local` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0412]: cannot find type `__OsLocalKeyInner` in module `$crate::thread`
   --> src/lib.rs:3:1
    |
3   | thread_local!(static LOCK_HELD: Cell<bool> = Cell::new(false));
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: a struct with a similar name exists: `__FastLocalKeyInner`
    |
   ::: /Users/sumukhgovindaraju/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/thread/local.rs:924:5
    |
924 |     pub struct Key<T> {
    |     ----------------- similarly named struct `__FastLocalKeyInner` defined here
    |
    = note: this error originates in the macro `$crate::__thread_local_inner` which comes from the expansion of the macro `thread_local` (in Nightly builds, run with -Z macro-backtrace for more info)

Some errors have detailed explanations: E0412, E0433.
For more information about an error, try `rustc --explain E0412`.
error: could not compile `hello_world` due to 2 previous errors
