plain
[01:48:24] test vec::test_splice_unbounded ... ok
[01:48:24] test vec::test_split_at_mut ... ok
[01:48:24] test vec::test_split_off ... ok
[01:48:24] test vec::test_swap_remove_empty ... ok
[01:48:25] died due to signal 11
[01:48:25] 
[01:48:25] 
[01:48:25] 
[01:48:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "arm-linux-androideabi" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--"
[01:48:25] 
[01:48:25] 
[01:48:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target arm-linux-androideabi
[01:48:25] Build completed unsuccessfully in 1:38:05
---
travis_time:end:27dfa8f8:start=1546807109913681851,finish=1546807109930439636,duration=16757785
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0fdbd5db
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04d4d9bc
travis_time:start:04d4d9bc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0465b8f8
$ dmesg | grep -i kill
