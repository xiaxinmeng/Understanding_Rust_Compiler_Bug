plain
[01:45:43] test string::test_str_truncate ... ok
[01:45:43] test string::test_str_truncate_invalid_len ... ok
[01:45:43] test string::test_str_truncate_split_codepoint ... ok
[01:45:43] test string::test_split_off_mid_char ... ok
[01:45:43] died due to signal 11
[01:45:43] error: test failed, to rerun pass '-p alloc --test collectionstests'
[01:45:43] 
[01:45:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "arm-linux-androideabi" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--"
[01:45:43] expected success, got: exit code: 3
[01:45:43] 
---
travis_time:end:184aeac4:start=1560457454522689465,finish=1560457454532226649,duration=9537184
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0e1f6da6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:009e3784
travis_time:start:009e3784
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1e6b56b6
$ dmesg | grep -i kill
