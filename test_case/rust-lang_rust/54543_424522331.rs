plain
[00:48:53]     Checking compiler_builtins v0.0.0 (file:///checkout/src/rustc/compiler_builtins_shim)
[00:48:55]  Documenting alloc v0.0.0 (file:///checkout/src/liballoc)
[00:48:55] thread 'main' panicked at '
[00:48:55] 
[00:48:55] failed to run "/path/to/nowhere/rustdoc/not/required" "--crate-name" "alloc" "liballoc/lib.rs" "--target" "x86_64-unknown-linux-gnu" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc" "index-page" "/checkout/src/doc/index.md" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps" "--extern" "compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-66b9d646ca55a4f6.rmeta" "--extern" "core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-dfbcb38dfdb6370a.rmeta" "--cfg" "stage1" "--cfg" "dox" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1" "-Z" "force-unstable-if-unmarked" "-Z" "unstable-options" "--crate-version" "1.30.0-dev": No such file or directory (os error 2)
[00:48:55] ', bootstrap/bin/rustdoc.rs:79:19
[00:48:55] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:55] error: Could not document `alloc`.
[00:48:55] 
[00:48:55] 
[00:48:55] Caused by:
[00:48:55]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name alloc liballoc/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-66b9d646ca55a4f6.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-dfbcb38dfdb6370a.rmeta` (exit code: 101)
[00:48:55] 
[00:48:55] 
[00:48:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-Z" "unstable-options" "-p" "alloc" "--" "index-page" "/checkout/src/doc/index.md"
[00:48:55] 
[00:48:55] 
[00:48:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[00:48:55] Build completed unsuccessfully in 0:05:22
[00:48:55] Build completed unsuccessfully in 0:05:22
[00:48:55] Makefile:28: recipe for target 'all' failed
[00:48:55] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0df531d4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:020afff3:start=1537914330481632417,finish=1537914330487494212,duration=5861795
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0f542d87
$ ln -s . checkout && for CORE in obj/cores/
