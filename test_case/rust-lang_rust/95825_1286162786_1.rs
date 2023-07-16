shell
$ rustc --version
rustc 1.61.0-nightly (38a0b81b1 2022-03-06)
$ RUSTFLAGS="-C instrument-coverage" cargo test --all-features
...
   Compiling atty v0.2.14
   Compiling dirs-sys v0.3.7
   Compiling tempfile v3.3.0
   Compiling clap v2.34.0
error[E0658]: trait bounds other than `Sized` on const fn parameters are unstable
   --> /home/dup2rt/.cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-epoch-0.9.8/src/atomic.rs:314:6
    |
314 | impl<T: ?Sized + Pointable> Atomic<T> {
    |      ^
...
346 |     pub const fn null() -> Atomic<T> {
    |     -------------------------------- function declared as const here
    |
    = note: see issue #93706 <https://github.com/rust-lang/rust/issues/93706> for more information
    = help: add `#![feature(const_fn_trait_bound)]` to the crate attributes to enable

   Compiling dirs v3.0.2
For more information about this error, try `rustc --explain E0658`.
error: could not compile `crossbeam-epoch` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
make: *** [Makefile:2: red] Error 101
