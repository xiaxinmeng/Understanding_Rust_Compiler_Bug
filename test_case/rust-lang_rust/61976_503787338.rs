plain
travis_time:end:13338fc0:start=1560983167396136177,finish=1560983168177490865,duration=781354688
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
[01:07:11] 
[01:07:11] running 9 tests
[01:07:11] iiiiiiiii
[01:07:11] 
[01:07:11]  finished in 0.152
[01:07:11] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:27] 
[01:07:27] running 122 tests
[01:07:53] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:07:58] .i.i......iii.i.....ii
[01:07:58] 
[01:07:58]  finished in 30.631
[01:07:58] travis_fold:end:test_debuginfo

---
[01:24:23] ..................................................i..i.......................................iiii... 400/927
[01:24:34] ....ii.............................................................................................. 500/927
[01:24:39] .................................................................................................... 600/927
[01:24:48] ..................................................iiii.............................................. 700/927
[01:25:03] .....................................................................F.............................. 800/927
[01:25:11] ...........................
[01:25:11] failures:
[01:25:11] 
[01:25:11] ---- sync/mutex.rs - sync::mutex::Mutex<T>::with (line 374) stdout ----
[01:25:11] ---- sync/mutex.rs - sync::mutex::Mutex<T>::with (line 374) stdout ----
[01:25:11] error[E0425]: cannot find value `old` in this scope
[01:25:11]  --> sync/mutex.rs:380:28
[01:25:11]   |
[01:25:11] 9 | let old = mutex.with(|v| { old = v; *v += 1; old }).unwrap();
[01:25:11] 
[01:25:11] error[E0425]: cannot find value `old` in this scope
[01:25:11]  --> sync/mutex.rs:380:46
[01:25:11]   |
[01:25:11]   |
[01:25:11] 9 | let old = mutex.with(|v| { old = v; *v += 1; old }).unwrap();
[01:25:11] 
[01:25:11] 
[01:25:11] error[E0658]: use of unstable library feature 'mutex_with'
[01:25:11]  --> sync/mutex.rs:380:17
[01:25:11]   |
[01:25:11] 9 | let old = mutex.with(|v| { old = v; *v += 1; old }).unwrap();
[01:25:11]   |
[01:25:11]   = note: for more information, see https://github.com/rust-lang/rust/issues/61974
[01:25:11]   = note: for more information, see https://github.com/rust-lang/rust/issues/61974
[01:25:11]   = help: add #![feature(mutex_with)] to the crate attributes to enable
[01:25:11] error: aborting due to 3 previous errors
[01:25:11] 
[01:25:11] Some errors have detailed explanations: E0425, E0658.
[01:25:11] For more information about an error, try `rustc --explain E0425`.
---
[01:25:11] 
[01:25:11] error: test failed, to rerun pass '--doc'
[01:25:11] 
[01:25:11] 
[01:25:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:25:11] 
[01:25:11] 
[01:25:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:25:11] Build completed unsuccessfully in 1:20:16
---
travis_time:end:18564150:start=1560988291969317982,finish=1560988291974093087,duration=4775105
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:014f6fae
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:12d1e080
travis_time:start:12d1e080
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:295635e4
$ dmesg | grep -i kill
