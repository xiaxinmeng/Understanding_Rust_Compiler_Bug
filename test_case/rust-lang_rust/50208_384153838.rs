plain
[00:02:58]    Compiling cc v1.0.10
[00:02:58]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[00:02:58]    Compiling unwind v0.0.0 (file:///checkout/src/libunwind)
[00:02:58]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:03:01] error[E0405]: cannot find trait `FusedIterator` in this scope
[00:03:01]    --> libcore/str/lossy.rs:184:6
[00:03:01]     |
[00:03:01] 184 | impl FusedIterator for Utf8LossyChunksIter {}
[00:03:01]     |      ^^^^^^^^^^^^^ not found in this scope
[00:03:01] help: possible candidate is found in another module, you can import it into scope
[00:03:01] 11  | use iter::traits::FusedIterator;
[00:03:01]     |
[00:03:01] 
[00:03:01]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[00:03:01]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[00:03:04] error[E0106]: missing lifetime specifier
[00:03:04]    --> libcore/str/lossy.rs:184:24
[00:03:04]     |
[00:03:04] 184 | impl FusedIterator for Utf8LossyChunksIter {}
[00:03:04]     |                        ^^^^^^^^^^^^^^^^^^^ expected lifetime parameter
[00:03:04] error: aborting due to 2 previous errors
[00:03:04] 
[00:03:04] Some errors occurred: E0106, E0405.
[00:03:04] For more information about an error, try `rustc --explain E0106`.
---
[00:03:05] travis_fold:end:stage0-std

[00:03:05] travis_time:end:stage0-std:start=1524628725710163325,finish=1524628733057800408,duration=7347637083

[00:03:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:05] expected success, got: exit code: 101
[00:03:05] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:03:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:05] Build completed unsuccessfully in 0:00:08
[00:03:05] Build completed unsuccessfully in 0:00:08
[00:03:05] make: *** [tidy] Error 1
[00:03:05] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:116c0c88
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
