plain
[00:17:56]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:17:57]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:17:57]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:17:58]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:18:42] error: could not parse/generate dep info at: /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/core-1cbcabaa1ea822b5.d
[00:18:42] To learn more, run the command again with --verbose.
[00:18:42] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:18:42] expected success, got: exit code: 101
[00:18:42] expected success, got: exit code: 101
[00:18:42] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:18:42] travis_fold:end:stage1-std

[00:18:42] travis_time:end:stage1-std:start=1531813986336194463,finish=1531814042293753182,duration=55957558719


[00:18:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:18:42] Build completed unsuccessfully in 0:15:06
[00:18:43] make: *** [all] Error 1
[00:18:43] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:122c7394
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:22a1ec40:start=1531814042867792741,finish=1531814042874915860,duration=7123119
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:15573fcc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:11b9ca10
travis_time:start:11b9ca10
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a73083e
$ dmesg | grep -i kill
