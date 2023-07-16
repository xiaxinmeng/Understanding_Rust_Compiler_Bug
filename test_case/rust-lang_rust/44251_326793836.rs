
[01:18:11] [m[m[32m[1m     Running[m build/x86_64-unknown-linux-gnu/stage2-std/i686-unknown-linux-musl/release/deps/coretests-41a06d4921b2cc6a
[01:18:11] 
[01:18:11] running 674 tests
[01:18:11] test any::any_downcast_mut ... ok
[01:18:11] test any::any_fixed_vec ... ok
...
[01:18:11] test num::test_try_u32usize ... ok
[01:18:11] test num::test_try_u64i128 ... ok
[01:18:11] [m[m[31m[1merror:[m An unknown error occurred
[01:18:11] 
[01:18:11] To learn more, run the command again with --verbose.
[01:18:11] 
[01:18:11] 
[01:18:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "-j" "4" "--target" "i686-unknown-linux-musl" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std:0.0.0" "-p" "alloc_system:0.0.0" "-p" "unwind:0.0.0" "-p" "collections:0.0.0" "-p" "panic_abort:0.0.0" "-p" "libc:0.0.0" "-p" "std_unicode:0.0.0" "-p" "compiler_builtins:0.0.0" "-p" "alloc:0.0.0" "-p" "core:0.0.0" "-p" "rand:0.0.0" "--"
[01:18:11] expected success, got: exit code: 101
[01:18:11] 
[01:18:11] 
[01:18:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i686-unknown-linux-musl --target i586-unknown-linux-gnu
[01:18:11] Build completed unsuccessfully in 1:16:14
