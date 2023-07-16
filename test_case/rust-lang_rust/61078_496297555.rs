plain
[01:42:44] test string::test_str_truncate ... ok
[01:42:44] test string::test_str_truncate_invalid_len ... ok
[01:42:44] test string::test_str_truncate_split_codepoint ... ok
[01:42:44] test str::test_unsafe_slice ... ok
[01:42:44] died due to signal 11
[01:42:44] error: test failed, to rerun pass '-p alloc --test collectionstests'
[01:42:44] 
[01:42:44] 
[01:42:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "arm-linux-androideabi" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--"
[01:42:44] 
[01:42:44] 
[01:42:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target arm-linux-androideabi
[01:42:44] Build completed unsuccessfully in 1:31:55
---
travis_time:end:0f441fa4:start=1558987816197147461,finish=1558987816206363980,duration=9216519
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06546b1c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0f6bfa15
travis_time:start:0f6bfa15
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:000bf496
$ dmesg | grep -i kill
