plain
[01:01:10] 
[01:01:11] error: unused import: `self::stdio::stderr_raw`
[01:01:11]    --> src/libstd/io/mod.rs:290:16
[01:01:11]     |
[01:01:11] 290 | pub(crate) use self::stdio::stderr_raw;
[01:01:11]     |
[01:01:11]     = note: `-D unused-imports` implied by `-D warnings`
[01:01:11] 
[01:01:15] error: aborting due to previous error
---
travis_time:end:2c346c21:start=1550894470605629984,finish=1550894470628201850,duration=22571866
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00afba0c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04cac408
travis_time:start:04cac408
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0604e0fc
$ dmesg | grep -i kill
