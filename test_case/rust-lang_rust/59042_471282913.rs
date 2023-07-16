plain
travis_time:end:0a3166b8:start=1552214022178409272,finish=1552214023072077099,duration=893667827
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:21:20] 
[01:21:20] running 120 tests
[01:21:45] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:21:49] .i......iii.i.....ii
[01:21:49] 
[01:21:49]  finished in 29.657
[01:21:49] travis_fold:end:test_debuginfo

---
[01:44:16] 
[01:44:16] running 9 tests
[01:44:16] thread 'rustc' panicked at 'index out of bounds: the len is 1 but the index is 4294967040', /checkout/src/libcore/slice/mod.rs:2539:10
[01:44:16] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:44:16] ...F.....
[01:44:16] 
[01:44:16] failures:
[01:44:16]     test::contravariant_region_ptr_err
[01:44:16] 
[01:44:16] 
[01:44:16] test result: FAILED. 8 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:44:16] 
[01:44:16] error: test failed, to rerun pass '--lib'
[01:44:16] 
[01:44:16] 
[01:44:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_driver" "--" "--quiet"
[01:44:16] 
[01:44:16] 
[01:44:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:44:16] Build completed unsuccessfully in 0:34:46
[01:44:16] Build completed unsuccessfully in 0:34:46
[01:44:16] Makefile:48: recipe for target 'check' failed
[01:44:16] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0ee3edb9
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Mar 10 12:18:09 UTC 2019
---
travis_time:end:00a85bd8:start=1552220291677336641,finish=1552220291682670297,duration=5333656
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0215f9dc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:060b5b5f
travis_time:start:060b5b5f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1c6e0980
$ dmesg | grep -i kill
