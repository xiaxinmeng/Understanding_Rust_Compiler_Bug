
---- ptr/mod.rs - ptr::drop_in_place (line 158) stdout ----
Test executable failed (exit code 101).

stderr:
thread 'main' panicked at 'attempt to copy from unaligned or null pointer', src/libcore/intrinsics.rs:1510:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.



failures:
    ptr/mod.rs - ptr::drop_in_place (line 158)
