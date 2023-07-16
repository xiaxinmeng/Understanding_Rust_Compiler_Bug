plain
[00:06:23] For more information about this error, try `rustc --explain E0425`.
[00:06:23] error: Could not compile `rustc_mir`.
[00:06:23] warning: build failed, waiting for other jobs to finish...
[00:06:28] error: build failed
[00:06:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:06:28] expected success, got: exit code: 101
[00:06:28] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1115:9
[00:06:28] travis_fold:end:stage0-rustc

[00:06:28] travis_time:end:stage0-rustc:start=1539877283060719757,finish=1539877418125808465,duration=135065088708

---
travis_time:end:0d17b91c:start=1539877418834613463,finish=1539877418841667840,duration=7054377
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2631c71a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1d4f1a15
travis_time:start:1d4f1a15
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:023e7a7c
$ dmesg | grep -i kill
