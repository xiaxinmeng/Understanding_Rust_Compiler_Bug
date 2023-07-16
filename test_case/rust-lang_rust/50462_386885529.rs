plain
[00:03:13]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:03:29]    Compiling libc v0.0.0 (file:///checkout/src/rustc/libc_shim)
[00:03:29]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[00:03:29]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:03:29] error[E0518]: attribute should be applied to function
[00:03:29]      |
[00:03:29] 2184 |   #[inline]
[00:03:29]      |   ^^^^^^^^^
[00:03:29]      |   ^^^^^^^^^
[00:03:29] 2185 |   #[stable(feature = "string_to_string_specialization", since = "1.17.0")]
[00:03:29] 2186 | / impl ToString for String {
[00:03:29] 2187 | |     #[must_use]
[00:03:29] 2188 | |     fn to_string(&self) -> String {
[00:03:29] 2190 | |     }
[00:03:29] 2191 | | }
[00:03:29] 2191 | | }
[00:03:29]      | |_- not a function
[00:03:30]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:03:30]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:03:32] error: aborting due to previous error
[00:03:32] 
[00:03:32] 
[00:03:32] For more information about this error, try `rustc --explain E0518`.
[00:03:32] error: Could not compile `alloc`.
[00:03:32] 
[00:03:32] Caused by:
[00:03:32]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name alloc liballoc/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=3 -C metadata=5c6ca57f52fc716b -C extra-filename=-5c6ca57f52fc716b --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-fb1e36473ec4786e.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-bad063b3019d016c.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-af41331a61619951/out` (exit code: 101)
[00:03:34] error: build failed
[00:03:34] error: build failed
[00:03:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:34] expected success, got: exit code: 101
[00:03:34] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:03:34] travis_fold:end:stage0-std

[00:03:34] travis_time:end:stage0-std:start=1525618708642499454,finish=1525618743514838858,duration=34872339404


[00:03:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:34] Build completed unsuccessfully in 0:00:36
[00:03:34] make: *** [tidy] Error 1
[00:03:34] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:21378f00
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
