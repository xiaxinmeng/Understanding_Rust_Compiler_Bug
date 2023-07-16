plain
[00:03:50]    Compiling unwind v0.0.0 (file:///checkout/src/libunwind)
[00:03:55] error[E0544]: multiple stability levels
[00:03:55]   --> libcore/time.rs:42:1
[00:03:55]    |
[00:03:55] 42 | #[unstable(feature = "time_units", issue = "0")]
[00:03:55] 
[00:03:56]    Compiling compiler_builtins v0.0.0 (file:///checkout/src/rustc/compiler_builtins_shim)
[00:03:56]    Compiling cmake v0.1.31
[00:03:56]    Compiling alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
---
travis_time:end:1badcc2e:start=1532079468001998045,finish=1532079468008235608,duration=6237563
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:089ca73e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0cf67578
travis_time:start:0cf67578
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6

