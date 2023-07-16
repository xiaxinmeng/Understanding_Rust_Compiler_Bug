
[01:06:09] thread '<unnamed>' panicked at 'explicit panic', /checkout/src/liballoc/../liballoc/tests/slice.rs:1194:16
...
[01:06:10] (B[m[31m[1merror:(B[m An unknown error occurred
[01:06:10] (B[m
[01:06:10] To learn more, run the command again with --verbose.
[01:06:10] 
[01:06:10] 
[01:06:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "-j" "4" "--target" "x86_64-unknown-linux-musl" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--features" "panic-unwind jemalloc backtrace" "-p" "std:0.0.0" "-p" "core:0.0.0" "-p" "unwind:0.0.0" "-p" "rustc_asan:0.0.0" "-p" "panic_abort:0.0.0" "-p" "rustc_tsan:0.0.0" "-p" "compiler_builtins:0.0.0" "-p" "rand:0.0.0" "-p" "rustc_lsan:0.0.0" "-p" "rustc_msan:0.0.0" "-p" "alloc_system:0.0.0" "-p" "std_unicode:0.0.0" "-p" "collections:0.0.0" "-p" "libc:0.0.0" "-p" "alloc:0.0.0" "--"
