
[01:21:56] test slice::test_capacity ... ok
[01:21:56] test slice::test_chunksator ... ok
[01:21:56] test slice::test_chunksator_0 ... ok
[01:21:56] thread '<unnamed>' panicked at 'explicit panic', /checkout/src/liballoc/../liballoc/tests/slice.rs:1194:16
[01:21:56] test slice::test_concat ... ok
[01:21:56] (B[m[31m[1merror:(B[m An unknown error occurred
[01:21:56] (B[m
[01:21:56] To learn more, run the command again with --verbose.
[01:21:56] 
[01:21:56] 
[01:21:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "-j" "4" "--target" "i686-unknown-linux-musl" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--features" "panic-unwind jemalloc backtrace" "-p" "std:0.0.0" "-p" "alloc:0.0.0" "-p" "compiler_builtins:0.0.0" "-p" "unwind:0.0.0" "-p" "core:0.0.0" "-p" "std_unicode:0.0.0" "-p" "libc:0.0.0" "-p" "alloc_system:0.0.0" "-p" "rustc_lsan:0.0.0" "-p" "rustc_asan:0.0.0" "-p" "panic_abort:0.0.0" "-p" "rustc_msan:0.0.0" "-p" "collections:0.0.0" "-p" "rustc_tsan:0.0.0" "-p" "rand:0.0.0" "--"
[01:21:56] expected success, got: exit code: 101
[01:21:56] 
[01:21:56] 
[01:21:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i686-unknown-linux-musl --target i586-unknown-linux-gnu
[01:21:56] Build completed unsuccessfully in 1:20:21
[01:21:56] (B[m
