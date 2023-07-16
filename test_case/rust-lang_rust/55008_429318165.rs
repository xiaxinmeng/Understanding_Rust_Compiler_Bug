plain
[00:21:43] 
[00:21:43] To learn more, run the command again with --verbose.
[00:21:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:21:43] expected success, got: exit code: 101
[00:21:43] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1112:9
[00:21:43] travis_fold:end:stage1-std

[00:21:43] travis_time:end:stage1-std:start=1539349376954259881,finish=1539349423332470265,duration=46378210384


[00:21:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:21:43] Build completed unsuccessfully in 0:17:21
[00:21:43] Makefile:28: recipe for target 'all' failed
[00:21:43] make: *** [all] Error 1
93748 ./src/tools/clang/test
76112 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot
76108 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib
76104 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib
---
22000 ./.git/modules/src/tools/cargo/objects
21992 ./.git/modules/src/tools/cargo/objects/pack
travis_time:end:00247668:start=1539349423645335975,finish=1539349424145716636,duration=500380661
travis_fold:end:after_failure.1
travis_fold:start:clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:04896950
$ dmesg | grep -i kill
