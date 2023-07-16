plain
[00:03:00]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[00:03:04]    Compiling compiler_builtins v0.0.0 (file:///checkout/src/rustc/compiler_builtins_shim)
[00:03:04]    Compiling cmake v0.1.30
[00:03:04]    Compiling alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
[00:03:08] error[E0231]: only named substitution parameters are allowed
[00:03:08]    --> libcore/fmt/mod.rs:611:1
[00:03:08]     |
[00:03:08] 611 | / #[rustc_on_unimplemented(
[00:03:08] 612 | |     message="`{Self}` doesn't implement `{Display}`",
[00:03:08] 613 | |     label="`{Self}` cannot be formatted with the default formatter; \
[00:03:08] 614 | |            try using `{:#?}` instead if you are using a format string",
[00:03:08] 615 | | )]
[00:03:08]     | |__^
[00:03:08] error: aborting due to previous error
[00:03:08] 
[00:03:08] For more information about this error, try `rustc --explain E0231`.
[00:03:08] error: Could not compile `core`.
[00:03:08] error: Could not compile `core`.
[00:03:08] 
[00:03:08] Caused by:
[00:03:08]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=3 -C metadata=fb1e36473ec4786e -C extra-filename=-fb1e36473ec4786e --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps` (exit code: 101)
[00:03:08] warning: build failed, waiting for other jobs to finish...
[00:03:10] error: build failed
[00:03:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:10] expected success, got: exit code: 101
[00:03:10] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:03:10] travis_fold:end:stage0-std

[00:03:10] travis_time:end:stage0-std:start=1525434932963736033,finish=1525434945811664640,duration=12847928607


[00:03:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:10] Build completed unsuccessfully in 0:00:14
[00:03:10] make: *** [tidy] Error 1
[00:03:10] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:13053696
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
