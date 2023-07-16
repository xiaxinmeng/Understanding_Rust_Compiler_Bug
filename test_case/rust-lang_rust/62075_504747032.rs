plain
[01:44:33] test string::test_str_truncate ... ok
[01:44:33] test string::test_str_truncate_invalid_len ... ok
[01:44:33] test string::test_retain ... ok
[01:44:33] test string::test_str_truncate_split_codepoint ... ok
[01:44:33] error: test failed, to rerun pass '-p alloc --test collectionstests'
[01:44:33] died due to signal 11
[01:44:33] 
[01:44:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "arm-linux-androideabi" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--"
[01:44:33] expected success, got: exit code: 3
[01:44:33] 
---
travis_time:end:01eb9dbb:start=1561292788302978447,finish=1561292788312777061,duration=9798614
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1e5bbdfb
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1c2e7180
travis_time:start:1c2e7180
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a19634c
$ dmesg | grep -i kill
