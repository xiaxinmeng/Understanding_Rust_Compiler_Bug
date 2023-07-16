plain
[01:47:20] test string::test_str_clear ... ok
[01:47:20] test string::test_str_truncate ... ok
[01:47:20] test string::test_str_truncate_invalid_len ... ok
[01:47:20] test string::test_str_truncate_split_codepoint ... ok
[01:47:20] died due to signal 11
[01:47:20] 
[01:47:20] 
[01:47:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "arm-linux-androideabi" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--"
[01:47:20] expected success, got: exit code: 3
---
travis_time:end:02951dea:start=1557405720635638569,finish=1557405720647107668,duration=11469099
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:170899cb
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:15577f37
$ dmesg | grep -i kill
