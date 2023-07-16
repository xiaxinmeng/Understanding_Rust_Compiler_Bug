
[01:11:45] [m[m[32m[1m     Running[m build/x86_64-unknown-linux-gnu/stage2-std/i686-unknown-linux-musl/release/deps/coretests-fe8a2762af9e798e
[01:11:45] 
[01:11:45] running 676 tests
...
[01:11:45] [m[m[31m[1merror:[m An unknown error occurred
[01:11:45] 
[01:11:45] To learn more, run the command again with --verbose.
[01:11:45] 
[01:11:45] 
[01:11:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "i686-unknown-linux-musl" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std:0.0.0" "-p" "panic_abort:0.0.0" "-p" "alloc:0.0.0" "-p" "libc:0.0.0" "-p" "rand:0.0.0" "-p" "unwind:0.0.0" "-p" "collections:0.0.0" "-p" "compiler_builtins:0.0.0" "-p" "core:0.0.0" "-p" "std_unicode:0.0.0" "-p" "alloc_system:0.0.0" "--"
[01:11:45] expected success, got: exit code: 101
[01:11:45] 
[01:11:45] 
[01:11:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i686-unknown-linux-musl --target i586-unknown-linux-gnu
[01:11:45] Build completed unsuccessfully in 1:09:49
