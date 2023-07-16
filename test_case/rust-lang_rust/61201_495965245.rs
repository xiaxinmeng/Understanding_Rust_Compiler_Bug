plain
[01:52:03] test string::test_str_truncate_invalid_len ... ok
[01:52:03] test string::test_str_truncate_split_codepoint ... ok
[01:52:03] test string::test_str_truncate ... ok
[01:52:04] error: test failed, to rerun pass '--test collectionstests'
[01:52:04] died due to signal 11
[01:52:04] 
[01:52:04] 
[01:52:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "arm-linux-androideabi" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--"
[01:52:04] 
[01:52:04] 
[01:52:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target arm-linux-androideabi
[01:52:04] Build completed unsuccessfully in 1:40:58
---
travis_time:end:116711f2:start=1558841098755959057,finish=1558841098771058312,duration=15099255
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2e604440
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1ad67710
travis_time:start:1ad67710
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:13261b89
$ dmesg | grep -i kill
