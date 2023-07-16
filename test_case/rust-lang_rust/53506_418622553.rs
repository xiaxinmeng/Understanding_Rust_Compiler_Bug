plain
[00:04:58]    Compiling cc v1.0.22
[00:04:58]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[00:04:58]    Compiling unwind v0.0.0 (file:///checkout/src/libunwind)
[00:04:58]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:04:59] error: incorrect close delimiter: `}`
[00:04:59]      |
[00:04:59] 1128 |             }
[00:04:59]      |             ^
[00:04:59]      |
---
[00:05:00]     --> libcore/sync/atomic.rs:1124:24
[00:05:00]      |
[00:05:00] 1124 |                   concat!(
[00:05:00]      |  ________________________^
[00:05:00] 1125 | | "Converts an [`", stringify!($int_type), "`] into an [`", stringify!($atomic_type), "`]."
[00:05:00] 1126 | |                 #[inline]
[00:05:00] 1127 | |                 fn from(v: $int_type) -> Self { Self::new(v) }
[00:05:00]      | |_____________^
[00:05:00] 
[00:05:01] error: aborting due to 2 previous errors
[00:05:01] 
---
travis_time:end:13c12a40:start=1536131192847074191,finish=1536131192856304931,duration=9230740
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2413b6de
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1be9925c
travis_time:start:1be9925c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:11b7e8b4
$ dmesg | grep -i kill
