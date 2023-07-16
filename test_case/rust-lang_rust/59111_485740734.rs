plain
travis_time:end:04f8f6e4:start=1556007801982773210,finish=1556007802737137811,duration=754364601
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
[01:14:51] 
[01:14:51] running 9 tests
[01:14:51] iiiiiiiii
[01:14:51] 
[01:14:51]  finished in 0.162
[01:14:51] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:15:07] 
[01:15:07] running 121 tests
[01:15:33] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:15:37] i.i......iii.i.....ii
[01:15:37] 
[01:15:37]  finished in 30.364
[01:15:37] travis_fold:end:test_debuginfo

---
[01:43:27] 
[01:43:27] failures:
[01:43:27] 
[01:43:27] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0698 (line 11243) stdout ----
[01:43:27] error[E0670]: `async fn` is not permitted in the 2015 edition
[01:43:27]   |
[01:43:27]   |
[01:43:27] 4 | async fn bar<T>() -> () {}
[01:43:27] 
[01:43:27] 
[01:43:27] error[E0670]: `async fn` is not permitted in the 2015 edition
[01:43:27]   |
[01:43:27]   |
[01:43:27] 6 | async fn foo() {
[01:43:27] 
[01:43:27] error: aborting due to 2 previous errors
[01:43:27] 
[01:43:27] For more information about this error, try `rustc --explain E0670`.
---
[01:43:27] 
[01:43:27] 
[01:43:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:43:27] Build completed unsuccessfully in 0:40:09
[01:43:27] Makefile:48: recipe for target 'check' failed
[01:43:27] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2e7c3890
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Apr 23 10:07:02 UTC 2019
---
travis_time:end:022a290a:start=1556014024082967818,finish=1556014024087678923,duration=4711105
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03636af8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:008db78d
travis_time:start:008db78d
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0b9f2290
$ dmesg | grep -i kill
