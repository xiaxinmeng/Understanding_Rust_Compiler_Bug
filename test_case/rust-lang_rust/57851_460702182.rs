plain
[01:41:51] test vec::test_split_at_mut ... ok
[01:41:51] test vec::test_split_off ... ok
[01:41:51] test vec::test_splice_unbounded ... ok
[01:41:51] test vec::test_swap_remove_empty ... ok
[01:41:52] died due to signal 11
[01:41:52] 
[01:41:52] 
[01:41:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "arm-linux-androideabi" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--"
[01:41:52] expected success, got: exit code: 3
---
travis_time:end:0047c8e0:start=1549383719637788139,finish=1549383719654481604,duration=16693465
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c8f1ed4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:110e7004
travis_time:start:110e7004
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0bd44166
$ dmesg | grep -i kill
