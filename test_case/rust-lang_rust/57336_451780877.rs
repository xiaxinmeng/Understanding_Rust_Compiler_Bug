plain
[01:43:10] test string::test_str_clear ... ok
[01:43:10] test string::test_str_truncate ... ok
[01:43:10] test string::test_str_truncate_split_codepoint ... ok
[01:43:10] test string::test_str_truncate_invalid_len ... ok
[01:43:11] died due to signal 11
[01:43:11] 
[01:43:11] 
[01:43:11] 
[01:43:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "arm-linux-androideabi" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--"
[01:43:11] 
[01:43:11] 
[01:43:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target arm-linux-androideabi
[01:43:11] Build completed unsuccessfully in 1:33:05
---
travis_time:end:0042a800:start=1546813527584428535,finish=1546813527608057338,duration=23628803
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05889417
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1506608e
travis_time:start:1506608e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:33cae7a2
$ dmesg | grep -i kill
