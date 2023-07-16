plain
[00:49:11] ....................................................................................................
[00:49:15] ....................................................................................................
[00:49:17] ...........i........................................................................................
[00:49:21] ....................................................................................................
[00:49:23] ............................................................iiiiiiiii...............................
[00:49:29] ....................................................................................................
[00:49:33] ....................................................................................................
[00:49:36] .........................................i..........................................................
[00:49:39] ..........................................................................................i.i..ii...
---
[01:13:31]    Compiling rustc_data_structures v0.0.0 (file:///checkout/src/librustc_data_structures)
[01:13:31] error: unused `#[macro_use]` import
[01:13:31]   --> librustc_data_structures/lib.rs:54:18
[01:13:31]    |
[01:13:31] 54 | #[cfg_attr(test, macro_use)]
[01:13:31]    |
[01:13:31]    = note: `-D unused-imports` implied by `-D warnings`
[01:13:31] 
[01:13:34] error: aborting due to previous error
[01:13:34] error: aborting due to previous error
[01:13:34] 
[01:13:34] error: Could not compile `rustc_data_structures`.
[01:13:34] 
[01:13:34] To learn more, run the command again with --verbose.
[01:13:34] 
[01:13:34] 
[01:13:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_data_structures" "--" "--quiet"
[01:13:34] 
[01:13:34] 
[01:13:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:34] Build completed unsuccessfully in 0:28:34
[01:13:34] Build completed unsuccessfully in 0:28:34
[01:13:34] Makefile:58: recipe for target 'check' failed
[01:13:34] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:139628b0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:242cd07c:start=1535653938444373942,finish=1535653938539826331,duration=95452389
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:16d087a0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:262dd939
$ dmesg | grep -i kill
