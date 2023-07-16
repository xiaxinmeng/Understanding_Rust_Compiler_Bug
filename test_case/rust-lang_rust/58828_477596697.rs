plain
[01:13:25] 
[01:13:26] error: hidden lifetime parameters in types are deprecated
[01:13:26]   --> src/libstd/sys/cloudabi/backtrace.rs:37:66
[01:13:26]    |
[01:13:26] 37 |         unsafe { uw::_Unwind_Backtrace(trace_fn, &mut cx as *mut Context as *mut libc::c_void) };
[01:13:26] 
[01:13:26] error: hidden lifetime parameters in types are deprecated
[01:13:26] error: hidden lifetime parameters in types are deprecated
[01:13:26]   --> src/libstd/sys/cloudabi/shims/os.rs:37:42
[01:13:26]    |
[01:13:26] 37 | pub fn split_paths(_unparsed: &OsStr) -> SplitPaths {
[01:13:26] 
[01:13:30] error: aborting due to 2 previous errors
[01:13:30] 
[01:13:30] [RUSTC-TIMING] std test:false 4.965
---
travis_time:end:03edeb3e:start=1553780016322377085,finish=1553780016331404201,duration=9027116
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:24137784
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0018a4d5
travis_time:start:0018a4d5
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0b0f404b
$ dmesg | grep -i kill
