plain
[00:08:13] 
[00:08:13] To learn more, run the command again with --verbose.
[00:08:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:08:13] expected success, got: exit code: 101
[00:08:13] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1112:9
[00:08:13] travis_fold:end:stage0-rustc

[00:08:13] travis_time:end:stage0-rustc:start=1538204258249848143,finish=1538204429150607837,duration=170900759694


[00:08:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:08:13] Build completed unsuccessfully in 0:03:46
[00:08:13] make: *** [all] Error 1
[00:08:13] Makefile:28: recipe for target 'all' failed
ers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01a5a0b8
$ dmesg | grep -i kill
