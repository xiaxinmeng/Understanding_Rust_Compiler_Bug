plain
[00:05:01]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:05:16] error[E0711]: feature `try_from` is declared stable since 1.36.0, but was previously declared stable since 1.34.0
[00:05:16]   --> src/libcore/array.rs:75:1
[00:05:16]    |
[00:05:16] 75 | #[stable(feature = "try_from", since = "1.36.0")]
[00:05:16] 
[00:05:16] error: aborting due to previous error
[00:05:16] 
[00:05:16] For more information about this error, try `rustc --explain E0711`.
---
travis_time:end:11e37dfe:start=1556415923161285422,finish=1556415923168176238,duration=6890816
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04bf50d2
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01b327d2
travis_time:start:01b327d2
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:039561f2
$ dmesg | grep -i kill
