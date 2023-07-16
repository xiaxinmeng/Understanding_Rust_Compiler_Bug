plain
[00:04:24]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:04:25]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:04:38]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:04:39]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:04:39] error[E0599]: no method named `poll_unpin` found for type `F` in the current scope
[00:04:39]    --> liballoc/boxed.rs:922:18
[00:04:39]     |
[00:04:39] 922 |         (**self).poll_unpin(cx)
[00:04:39] 
[00:04:41] error: aborting due to previous error
[00:04:41] 
[00:04:41] For more information about this error, try `rustc --explain E0599`.
[00:04:41] For more information about this error, try `rustc --explain E0599`.
[00:04:41] error: Could not compile `alloc`.
[00:04:41] 
[00:04:41] Caused by:
[00:04:41]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name alloc liballoc/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=d6cd5f8b78fddf12 -C extra-filename=-d6cd5f8b78fddf12 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-e9cdce497aae9e81.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-90a13cda2e54742f.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-ffd422941bf53e42/out` (exit code: 101)
[00:04:41] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:04:41] expected success, got: exit code: 101
[00:04:41] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:04:41] travis_fold:end:stage0-std

[00:04:41] travis_time:end:stage0-std:start=1528492027441753057,finish=1528492062409231449,duration=34967478392


[00:04:41] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:41] Build completed unsuccessfully in 0:00:36
[00:04:41] make: *** [tidy] Error 1
[00:04:41] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:09db6b85
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
