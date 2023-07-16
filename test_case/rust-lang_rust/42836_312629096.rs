
[00:59:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "-j" "4" "--target" "x86_64-unknown-linux-musl" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--features" "panic-unwind jemalloc backtrace" "-p" "std:0.0.0" "-p" "rustc_lsan:0.0.0" "-p" "std_unicode:0.0.0" "-p" "collections:0.0.0" "-p" "unwind:0.0.0" "-p" "alloc:0.0.0" "-p" "rustc_tsan:0.0.0" "-p" "compiler_builtins:0.0.0" "-p" "libc:0.0.0" "-p" "panic_abort:0.0.0" "-p" "rand:0.0.0" "-p" "core:0.0.0" "-p" "rustc_msan:0.0.0" "-p" "rustc_asan:0.0.0" "-p" "alloc_system:0.0.0" "--"
[00:59:34] expected success, got: exit code: 101
[00:59:34] 
[00:59:34] 
[00:59:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target x86_64-unknown-linux-musl
[00:59:34] Build completed unsuccessfully in 0:58:11
