plain
[00:02:30]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[00:02:34]    Compiling compiler_builtins v0.0.0 (file:///checkout/src/rustc/compiler_builtins_shim)
[00:02:34]    Compiling cmake v0.1.30
[00:02:34]    Compiling alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
[00:02:37] error[E0053]: method `call_once` has an incompatible type for trait
[00:02:37]     --> libcore/str/mod.rs:4407:66
[00:02:37]      |
[00:02:37] 4407 |     extern "rust-call" fn call_once(mut self, arg: (&[u8], )) -> bool {
[00:02:37]      |                                                                  ^^^^ expected &str, found bool
[00:02:37]     ::: libcore/ops/function.rs:223:58
[00:02:37]      |
[00:02:37]      |
[00:02:37] 223  |     extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
[00:02:37]      |                                                          ------------ type in trait
[00:02:37]      |
[00:02:37]      = note: expected type `extern "rust-call" fn(str::BytesToStr, (&'a [u8],)) -> &str`
[00:02:37]                 found type `extern "rust-call" fn(str::BytesToStr, (&[u8],)) -> bool`
[00:02:37] 
[00:02:37] error[E0053]: method `call_mut` has an incompatible type for trait
[00:02:37]     --> libcore/str/mod.rs:4414:66
[00:02:37]      |
[00:02:37] 4414 |     extern "rust-call" fn call_mut(&mut self, arg: (&[u8], )) -> bool {
[00:02:37]      |                                                                  ^^^^ expected &str, found bool
[00:02:37]     ::: libcore/ops/function.rs:146:62
[00:02:37]      |
[00:02:37]      |
[00:02:37] 146  |     extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
[00:02:37]      |                                                              ------------ type in trait
[00:02:37]      |
[00:02:37]      = note: expected type `extern "rust-call" fn(&mut str::BytesToStr, (&'a [u8],)) -> &str`
[00:02:37]                 found type `extern "rust-call" fn(&mut str::BytesToStr, (&[u8],)) -> bool`
[00:02:38] error: aborting due to 2 previous errors
[00:02:38] 
[00:02:38] For more information about this error, try `rustc --explain E0053`.
[00:02:38] error: Could not compile `core`.
[00:02:38] error: Could not compile `core`.
[00:02:38] 
[00:02:38] Caused by:
[00:02:38]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=3 -C metadata=fb1e36473ec4786e -C extra-filename=-fb1e36473ec4786e --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps` (exit code: 101)
[00:02:38] warning: build failed, waiting for other jobs to finish...
[00:02:39] error: build failed
[00:02:39] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:02:39] expected success, got: exit code: 101
[00:02:39] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:02:39] travis_fold:end:stage0-std

[00:02:39] travis_time:end:stage0-std:start=1525494993941789505,finish=1525495005506178532,duration=11564389027


[00:02:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:02:39] Build completed unsuccessfully in 0:00:12
[00:02:39] Makefile:79: recipe for target 'tidy' failed
[00:02:39] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:061a7dda
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
