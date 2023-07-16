plain
[01:49:34] test string::test_str_clear ... ok
[01:49:34] test string::test_str_truncate ... ok
[01:49:34] test string::test_str_truncate_invalid_len ... ok
[01:49:34] test string::test_str_truncate_split_codepoint ... ok
[01:49:34] died due to signal 11
[01:49:34] 
[01:49:34] 
[01:49:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "arm-linux-androideabi" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--"
[01:49:34] expected success, got: exit code: 3
---
travis_time:end:08504cb5:start=1557796268113303392,finish=1557796268127153888,duration=13850496
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0234c5fe
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:294798ee
travis_time:start:294798ee
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02e1fc00
$ dmesg | grep -i kill
