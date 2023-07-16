plain
[00:03:02]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[00:03:06]    Compiling compiler_builtins v0.0.0 (file:///checkout/src/rustc/compiler_builtins_shim)
[00:03:06]    Compiling cmake v0.1.30
[00:03:06]    Compiling alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
[00:03:09] error[E0495]: cannot infer an appropriate lifetime for lifetime parameter in generic type due to conflicting requirements
[00:03:09]     --> libcore/str/mod.rs:4414:5
[00:03:09]      |
[00:03:09] 4414 |     extern "rust-call" fn call_mut(&mut self, arg: (&[u8], )) -> &str {
[00:03:09]      |
[00:03:09]      |
[00:03:09] note: first, the lifetime cannot outlive the anonymous lifetime #1 defined on the method body at 4414:5...
[00:03:09]     --> libcore/str/mod.rs:4414:5
[00:03:09]      |
[00:03:09] 4414 | /     extern "rust-call" fn call_mut(&mut self, arg: (&[u8], )) -> &str {
[00:03:09] 4415 | |         unsafe { from_utf8_unchecked(arg.0) }
[00:03:09]      | |_____^
[00:03:09]      | |_____^
[00:03:09]      = note: ...so that the method type is compatible with trait:
[00:03:09]              expected extern "rust-call" fn(&mut str::BytesToStr, (&'a [u8],)) -> &str
[00:03:09]                 found extern "rust-call" fn(&mut str::BytesToStr, (&[u8],)) -> &str
[00:03:09] note: but, the lifetime must be valid for the lifetime 'a as defined on the impl at 4412:1...
[00:03:09]     --> libcore/str/mod.rs:4412:1
[00:03:09]      |
[00:03:09] 4412 | impl<'a> FnMut<(&'a [u8], )> for BytesToStr {
[00:03:09]      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:03:09]      = note: ...so that the types are compatible:
[00:03:09]              expected ops::function::FnOnce<(&'a [u8],)>
[00:03:09]                 found ops::function::FnOnce<(&[u8],)>
[00:03:10] error: aborting due to previous error
[00:03:10] 
[00:03:10] For more information about this error, try `rustc --explain E0495`.
[00:03:10] error: Could not compile `core`.
[00:03:10] error: Could not compile `core`.
[00:03:10] 
[00:03:10] Caused by:
[00:03:10]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=3 -C metadata=fb1e36473ec4786e -C extra-filename=-fb1e36473ec4786e --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps` (exit code: 101)
[00:03:10] warning: build failed, waiting for other jobs to finish...
[00:03:11] error: build failed
[00:03:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:11] expected success, got: exit code: 101
[00:03:11] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:03:11] travis_fold:end:stage0-std

[00:03:11] travis_time:end:stage0-std:start=1525536829517741469,finish=1525536842688385075,duration=13170643606


[00:03:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:11] Build completed unsuccessfully in 0:00:14
[00:03:11] make: *** [tidy] Error 1
[00:03:11] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0963e966
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
