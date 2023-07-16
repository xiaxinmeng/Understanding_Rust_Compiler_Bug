plain
[00:21:28]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:21:47] error: local variables in const fn are unstable
[00:21:47]     --> libcore/ptr.rs:2879:17
[00:21:47]      |
[00:21:47] 2879 |             let ptr = mem::align_of::<T>() as *mut T;
[00:21:47] 
[00:21:52] error: aborting due to 2 previous errors
[00:21:52] 
[00:21:52] error: Could not compile `core`.
[00:21:52] error: Could not compile `core`.
[00:21:52] 
[00:21:52] To learn more, run the command again with --verbose.
[00:21:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:21:52] expected success, got: exit code: 101
[00:21:52] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1112:9
[00:21:52] travis_fold:end:stage1-std

[00:21:52] travis_time:end:stage1-std:start=1538181964619677157,finish=1538182001877925688,duration=37258248531


[00:21:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:21:52] Build completed unsuccessfully in 0:17:19
[00:21:52] Makefile:28: recipe for target 'all' failed
[00:21:52] make: *** [all] Error 1
1496488 ./obj
1496452 ./obj/build
1194916 ./.git
1068648 ./src
---
151412 ./src/tools/clang
149112 ./src/llvm-emscripten/test
148656 ./obj/build/bootstrap/debug/incremental
134216 ./obj/build/bootstrap/debug/incremental/bootstrap-2zc4gzhr0d54q
134212 ./obj/build/bootstrap/debug/incremental/bootstrap-2zc4gzhr0d54q/s-f58miyl5yx-42k002-abll1u66rzrn
104700 ./src/tools/lldb
98940 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
93748 ./src/tools/clang/test
93016 ./obj/build/x86_64-unknown-linux-gnu/stage1
