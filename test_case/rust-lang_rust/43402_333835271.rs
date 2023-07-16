
$ echo 'fn main() {}' | rustup run nightly rustc -o "" -
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: FatalError', /checkout/src/libcore/result.rs:906:4
note: Run with `RUST_BACKTRACE=1` for a backtrace.
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: FatalError', /checkout/src/libcore/result.rs:906:4
error: could not write output to : No such file or directory

error: aborting due to previous error

thread '<unnamed>' panicked at 'aborting due to worker thread panic', /checkout/src/librustc_trans/back/write.rs:1643:20
