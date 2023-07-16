
# echo 'fn main(){}' | rustc - -o true
# nm ./true | grep atexit
         U __cxa_atexit@@GLIBC_2.1.3
         w __cxa_thread_atexit_impl
00034f20 t atexit
000171c0 t stats_print_atexit
# ./true
thread '<unnamed>' panicked at 'cannot access a TLS value during or after it is destroyed', /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libcore/option.rs:715
note: Run with `RUST_BACKTRACE=1` for a backtrace.
fatal runtime error: failed to initiate panic, error 5
Aborted
