plain
travis_time:end:0f3d16ce:start=1560988541798627538,finish=1560988542542871746,duration=744244208
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:19] 
[01:05:19] running 9 tests
[01:05:19] iiiiiiiii
[01:05:19] 
[01:05:19]  finished in 0.155
[01:05:19] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:35] 
[01:05:35] running 122 tests
[01:06:00] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:06:05] .i.i......iii.i.....ii
[01:06:05] 
[01:06:05]  finished in 29.934
[01:06:05] travis_fold:end:test_debuginfo

---
[01:22:13] ..................................................i..i.......................................iiii... 400/927
[01:22:23] ....ii.............................................................................................. 500/927
[01:22:27] .................................................................................................... 600/927
[01:22:36] ..................................................iiii.............................................. 700/927
[01:22:50] .....................................................................F.............................. 800/927
[01:22:59] ...........................
[01:22:59] failures:
[01:22:59] 
[01:22:59] ---- sync/mutex.rs - sync::mutex::Mutex<T>::with (line 374) stdout ----
[01:22:59] ---- sync/mutex.rs - sync::mutex::Mutex<T>::with (line 374) stdout ----
[01:22:59] error[E0658]: use of unstable library feature 'mutex_with'
[01:22:59]  --> sync/mutex.rs:380:17
[01:22:59]   |
[01:22:59] 9 | let old = mutex.with(|v| { let old = v; *v += 1; old }).unwrap();
[01:22:59]   |
[01:22:59]   = note: for more information, see https://github.com/rust-lang/rust/issues/61974
[01:22:59]   = note: for more information, see https://github.com/rust-lang/rust/issues/61974
[01:22:59]   = help: add #![feature(mutex_with)] to the crate attributes to enable
[01:22:59] 
[01:22:59] error[E0277]: can't compare `&mut {integer}` with `{integer}`
[01:22:59]   --> sync/mutex.rs:382:1
[01:22:59] 11 | assert_eq!(old, 0);
[01:22:59] 11 | assert_eq!(old, 0);
[01:22:59]    | ^^^^^^^^^^^^^^^^^^^ no implementation for `&mut {integer} == {integer}`
[01:22:59]    |
[01:22:59]    = help: the trait `std::cmp::PartialEq<{integer}>` is not implemented for `&mut {integer}`
[01:22:59] 
[01:22:59] error: aborting due to 2 previous errors
[01:22:59] 
[01:22:59] Some errors have detailed explanations: E0277, E0658.
---
[01:22:59] 
[01:22:59] error: test failed, to rerun pass '--doc'
[01:22:59] 
[01:22:59] 
[01:22:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:22:59] 
[01:22:59] 
[01:22:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:22:59] Build completed unsuccessfully in 1:18:14
---
travis_time:end:148c203c:start=1560993534474586616,finish=1560993534479135248,duration=4548632
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:27c2d3ec
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1056ce70
travis_time:start:1056ce70
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1f903f0a
$ dmesg | grep -i kill
