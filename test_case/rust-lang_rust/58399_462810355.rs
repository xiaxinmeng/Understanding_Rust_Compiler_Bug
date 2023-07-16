plain
travis_time:end:0c41e893:start=1549981032816462754,finish=1549981033780339075,duration=963876321
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:09:19] 
[01:09:19] running 119 tests
[01:09:44] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:09:48] i......iii.i.....ii
[01:09:48] 
[01:09:48]  finished in 29.660
[01:09:48] travis_fold:end:test_debuginfo

---
[01:25:38] travis_fold:start:test_stage1-std
travis_time:start:test_stage1-std
Testing std stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:25:38]    Compiling std v0.0.0 (/checkout/src/libstd)
[01:25:50] error[E0599]: no method named `as_inner` found for type `&realstd::thread::JoinHandle<T>` in the current scope
[01:25:50]   --> src/libstd/sys/unix/ext/thread.rs:35:14
[01:25:50]    |
[01:25:50] 35 |         self.as_inner().id() as RawPthread
[01:25:50]    |
[01:25:50]    = help: items from traits can only be used if the trait is in scope
[01:25:50]    = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:25:50]    = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:25:50]            `use realstd::sys_common::AsInner;`
[01:25:50] 
[01:25:50] error[E0599]: no method named `into_inner` found for type `realstd::thread::JoinHandle<T>` in the current scope
[01:25:50]   --> src/libstd/sys/unix/ext/thread.rs:39:14
[01:25:50]    |
[01:25:50] 39 |         self.into_inner().into_id() as RawPthread
[01:25:50]    |
[01:25:50]    = help: items from traits can only be used if the trait is in scope
[01:25:50]    = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:25:50]    = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:25:50]            `use realstd::sys_common::IntoInner;`
[01:25:50] error: aborting due to 2 previous errors
[01:25:50] 
[01:25:50] For more information about this error, try `rustc --explain E0599`.
[01:25:50] error: Could not compile `std`.
[01:25:50] error: Could not compile `std`.
[01:25:50] 
[01:25:50] To learn more, run the command again with --verbose.
[01:25:50] 
[01:25:50] 
[01:25:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:25:50] 
[01:25:50] 
[01:25:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:25:50] Build completed unsuccessfully in 0:28:13
[01:25:50] Build completed unsuccessfully in 0:28:13
[01:25:50] make: *** [check] Error 1
[01:25:50] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1f331595
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb 12 15:43:15 UTC 2019
---
travis_time:end:013c56ae:start=1549986197309356229,finish=1549986197314499681,duration=5143452
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00868346
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2a293754
travis_time:start:2a293754
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0bab5b66
$ dmesg | grep -i kill
