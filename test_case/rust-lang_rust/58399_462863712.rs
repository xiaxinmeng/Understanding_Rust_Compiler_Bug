plain
travis_time:end:00810a80:start=1549988685461705896,finish=1549988686423445690,duration=961739794
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
[01:10:20] 
[01:10:20] running 119 tests
[01:10:45] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:10:49] i......iii.i.....ii
[01:10:49] 
[01:10:49]  finished in 28.907
[01:10:49] travis_fold:end:test_debuginfo

---
[01:27:00]    |
[01:27:00] 82 | #[allow_internal_unstable]
[01:27:00]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:27:00] 
[01:27:12] error[E0599]: no method named `as_inner` found for type `&realstd::thread::JoinHandle<T>` in the current scope
[01:27:12]   --> src/libstd/sys/unix/ext/thread.rs:35:14
[01:27:12]    |
[01:27:12] 35 |         self.as_inner().id() as RawPthread
[01:27:12]    |
[01:27:12]    = help: items from traits can only be used if the trait is in scope
[01:27:12]    = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:27:12]    = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:27:12]            `use realstd::sys_common::AsInner;`
[01:27:12] 
[01:27:12] error[E0599]: no method named `into_inner` found for type `realstd::thread::JoinHandle<T>` in the current scope
[01:27:12]   --> src/libstd/sys/unix/ext/thread.rs:39:14
[01:27:12]    |
[01:27:12] 39 |         self.into_inner().into_id() as RawPthread
[01:27:12]    |
[01:27:12]    = help: items from traits can only be used if the trait is in scope
[01:27:12]    = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:27:12]    = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:27:12]            `use realstd::sys_common::IntoInner;`
[01:27:12] error: aborting due to 2 previous errors
[01:27:12] 
[01:27:12] For more information about this error, try `rustc --explain E0599`.
[01:27:12] error: Could not compile `std`.
[01:27:12] error: Could not compile `std`.
[01:27:12] 
[01:27:12] To learn more, run the command again with --verbose.
[01:27:12] 
[01:27:12] 
[01:27:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:27:12] 
[01:27:12] 
[01:27:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:27:12] Build completed unsuccessfully in 0:28:41
[01:27:12] Build completed unsuccessfully in 0:28:41
[01:27:12] make: *** [check] Error 1
[01:27:12] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00170f4e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb 12 17:52:09 UTC 2019
---
travis_time:end:073fce48:start=1549993931372101060,finish=1549993931435820440,duration=63719380
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:10271d7e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0271a0e2
$ dmesg | grep -i kill
