plain
[00:09:01] [RUSTC-TIMING] syntax_ext test:false 38.888
[00:09:20] error[E0080]: could not evaluate static initializer
[00:09:20]   --> src/librustc/mir/interpret/value.rs:90:1
[00:09:20]    |
[00:09:20] 90 | static_assert!(SCALAR_SIZE: ::std::mem::size_of::<Scalar>() == 24);
[00:09:20]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ index out of bounds: the len is 1 but the index is 1
[00:09:20]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:09:20] 
[00:09:21] error: aborting due to previous error
[00:09:21] 
---
travis_time:end:08617a90:start=1547334683766495006,finish=1547334683772826171,duration=6331165
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02bf8978
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2dc77586
travis_time:start:2dc77586
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:20c6be36
$ dmesg | grep -i kill
