
$ rustc -C target-feature=+avx rust42515.rs
$ ./rust42515 
thread 'main' panicked at 'have_avx', rust42515.rs:6
note: Run with `RUST_BACKTRACE=1` for a backtrace.
