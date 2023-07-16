plain
travis_time:end:155f79e0:start=1551708531382670248,finish=1551708532305729200,duration=923058952
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
[01:21:28] 
[01:21:28] running 119 tests
[01:21:55] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:21:59] i......iii.i.....ii
[01:21:59] 
[01:21:59]  finished in 31.395
[01:21:59] travis_fold:end:test_debuginfo

---
[01:38:58] travis_fold:start:test_stage1-std
travis_time:start:test_stage1-std
Testing std stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:38:59]    Compiling std v0.0.0 (/checkout/src/libstd)
[01:39:01] error[E0433]: failed to resolve: use of undeclared type or module `RevBufReader`
[01:39:01]      |
[01:39:01]      |
[01:39:01] 1075 |         let mut reader = RevBufReader::with_capacity(3, io::Cursor::new(inner));
[01:39:01]      |                          ^^^^^^^^^^^^ use of undeclared type or module `RevBufReader`
[01:39:13] error: aborting due to previous error
[01:39:13] 
[01:39:13] For more information about this error, try `rustc --explain E0433`.
[01:39:13] error: Could not compile `std`.
[01:39:13] error: Could not compile `std`.
[01:39:13] 
[01:39:13] To learn more, run the command again with --verbose.
[01:39:13] 
[01:39:13] 
[01:39:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:39:13] 
[01:39:13] 
[01:39:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:39:13] Build completed unsuccessfully in 0:29:57
[01:39:13] Build completed unsuccessfully in 0:29:57
[01:39:13] Makefile:48: recipe for target 'check' failed
[01:39:13] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0ced8d38
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Mar  4 15:48:16 UTC 2019
Mon Mar  4 15:48:16 UTC 2019
echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:14fcb930
travis_time:start:14fcb930
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0d08b418
$ dmesg | grep -i kill
