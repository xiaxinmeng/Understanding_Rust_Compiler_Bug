plain
[00:04:01]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:04:13]    Compiling libc v0.0.0 (file:///checkout/src/rustc/libc_shim)
[00:04:13]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[00:04:13]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:04:13] error[E0433]: failed to resolve. Use of undeclared type or module `task`
[00:04:13]    --> liballoc/boxed.rs:921:46
[00:04:13]     |
[00:04:13] 921 |     fn poll(mut self: PinMut<Self>, cx: &mut task::Context) -> Poll<Self::Output> {
[00:04:13]     |                                              ^^^^ Use of undeclared type or module `task`
[00:04:13] 
[00:04:13] error[E0433]: failed to resolve. Use of undeclared type or module `task`
[00:04:13]    --> liballoc/boxed.rs:930:46
[00:04:13]     |
[00:04:13] 930 |     fn poll(mut self: PinMut<Self>, cx: &mut task::Context) -> Poll<Self::Output> {
[00:04:13]     |                                              ^^^^ Use of undeclared type or module `task`
[00:04:14]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:04:14]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:04:14] error[E0658]: arbitrary `self` types are unstable (see issue #44874)
[00:04:14]    --> liballoc/boxed.rs:921:23
[00:04:14]     |
[00:04:14] 921 |     fn poll(mut self: PinMut<Self>, cx: &mut task::Context) -> Poll<Self::Output> {
[00:04:14]     |
[00:04:14]     |
[00:04:14]     = help: add #![feature(arbitrary_self_types)] to the crate attributes to enable
[00:04:14]     = help: consider changing to `self`, `&self`, `&mut self`, or `self: Box<Self>`
[00:04:14] 
[00:04:14] error[E0658]: arbitrary `self` types are unstable (see issue #44874)
[00:04:14]    --> liballoc/boxed.rs:930:23
[00:04:14]     |
[00:04:14] 930 |     fn poll(mut self: PinMut<Self>, cx: &mut task::Context) -> Poll<Self::Output> {
[00:04:14]     |
[00:04:14]     |
[00:04:14]     = help: add #![feature(arbitrary_self_types)] to the crate attributes to enable
[00:04:14]     = help: consider changing to `self`, `&self`, `&mut self`, or `self: Box<Self>`
[00:04:14] error: aborting due to 4 previous errors
[00:04:14] 
[00:04:14] Some errors occurred: E0433, E0658.
[00:04:14] For more information about an error, try `rustc --explain E0433`.
[00:04:14] For more information about an error, try `rustc --explain E0433`.
[00:04:14]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:04:14] error: Could not compile `alloc`.
[00:04:14] 
[00:04:14] Caused by:
[00:04:14]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name alloc liballoc/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=d6cd5f8b78fddf12 -C extra-filename=-d6cd5f8b78fddf12 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-90a13cda2e54742f.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-e9cdce497aae9e81.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-ffd422941bf53e42/out` (exit code: 101)
[00:04:14] error: build failed
[00:04:14] error: build failed
[00:04:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:04:14] expected success, got: exit code: 101
[00:04:14] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:04:14] travis_fold:end:stage0-std

[00:04:14] travis_time:end:stage0-std:start=1528491391768457388,finish=1528491419482079125,duration=27713621737


[00:04:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:14] Build completed unsuccessfully in 0:00:28
[00:04:14] make: *** [tidy] Error 1
[00:04:14] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e92b086
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
