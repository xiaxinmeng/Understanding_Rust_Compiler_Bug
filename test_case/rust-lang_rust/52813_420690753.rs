plain
[01:12:31] .................................................................................................... 2000/2165
[01:12:41] ...................................................................................i................ 2100/2165
butes to enable
[01:12:46] 
[01:12:46] thread 'time.rs - time::Duration::div_f64 (line 500)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:12:46] 
[01:12:46] ---- time.rs - time::Duration::mul_f64 (line 466) stdout ----
[01:12:46] error[E0658]: use of unstable library feature 'duration_float_ops': duration/floats operations are unstabe
[01:12:46]  --> time.rs:471:16
[01:12:46]   |
[01:12:46] 8 | assert_eq!(dur.mul_f64(3.14), Duration::new(8, 478_000_000));
[01:12:46]   |
[01:12:46]   |
[01:12:46]   = help: add #![feature(duration_float_ops)] to the crate attributes to enable
[01:12:46] 
[01:12:46] error[E0658]: use of unstable library feature 'duration_float_ops': duration/floats operations are unstabe
[01:12:46]  --> time.rs:472:16
[01:12:46]   |
[01:12:46] 9 | assert_eq!(dur.mul_f64(3.14e5), Duration::new(847_800, 0));
[01:12:46]   |
[01:12:46]   |
[01:12:46]   = help: add #![feature(duration_float_ops)] to the crate attributes to enable
[01:12:46] 
[01:12:46] thread 'time.rs - time::Duration::mul_f64 (line 466)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:12:46] 
[01:12:46] failures:
[01:12:46]     time.rs - time::Duration::div_duration (line 535)
[01:12:46]     time.rs - time::Duration::div_f64 (line 500)
---
travis_time:end:00f602ca:start=1536765984942529025,finish=1536765984947629405,duration=5100380
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b98863f
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:18397cd0
travis_time:start:18397cd0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:20387268
$ dmesg | grep -i kill
