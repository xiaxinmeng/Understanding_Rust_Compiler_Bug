plain
travis_time:end:1bdb10b2:start=1561010002223205726,finish=1561010091696714849,duration=89473509123
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
[01:03:26] 
[01:03:26] running 9 tests
[01:03:26] iiiiiiiii
[01:03:26] 
[01:03:26]  finished in 0.145
[01:03:26] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:42] 
[01:03:42] running 122 tests
[01:04:06] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:04:11] .i.i......iii.i.....ii
[01:04:11] 
[01:04:11]  finished in 28.916
[01:04:11] travis_fold:end:test_debuginfo

---
[01:19:49] ..................................................i..i.......................................iiii... 400/927
[01:19:59] ....ii.............................................................................................. 500/927
[01:20:03] .................................................................................................... 600/927
[01:20:12] ..................................................iiii.............................................. 700/927
[01:20:27] ......................................................................F............................. 800/927
[01:20:35] ...........................
[01:20:35] failures:
[01:20:35] 
[01:20:35] ---- sync/mutex.rs - sync::mutex::Mutex<T>::with (line 374) stdout ----
[01:20:35] ---- sync/mutex.rs - sync::mutex::Mutex<T>::with (line 374) stdout ----
[01:20:35] error[E0658]: use of unstable library feature 'mutex_with'
[01:20:35]  --> sync/mutex.rs:380:17
[01:20:35]   |
[01:20:35] 9 | let old = mutex.with(|v| { let old = *v; *v += 1; old }).unwrap();
[01:20:35]   |
[01:20:35]   = note: for more information, see https://github.com/rust-lang/rust/issues/61974
[01:20:35]   = note: for more information, see https://github.com/rust-lang/rust/issues/61974
[01:20:35]   = help: add #![feature(mutex_with)] to the crate attributes to enable
[01:20:35] error: aborting due to previous error
[01:20:35] 
[01:20:35] For more information about this error, try `rustc --explain E0658`.
[01:20:35] Couldn't compile the test.
---
[01:20:35] 
[01:20:35] error: test failed, to rerun pass '--doc'
[01:20:35] 
[01:20:35] 
[01:20:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:20:35] 
[01:20:35] 
[01:20:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:20:35] Build completed unsuccessfully in 1:16:37
---
travis_time:end:2c39fef8:start=1561014937184572312,finish=1561014937189228075,duration=4655763
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:10af183d
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:152cc5ce
travis_time:start:152cc5ce
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03269352
$ dmesg | grep -i kill
