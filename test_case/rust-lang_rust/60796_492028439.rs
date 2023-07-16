plain
travis_time:end:03b05880:start=1557786640867044425,finish=1557786642982710079,duration=2115665654
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:17:00] 
[01:17:00] running 143 tests
[01:17:03] i..iii.....iii..iiii.....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/143
[01:17:05] test result: ok. 113 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:17:05] 
[01:17:05]  finished in 4.535
[01:17:05] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:17:07] 
[01:17:07] running 9 tests
[01:17:07] iiiiiiiii
[01:17:07] 
[01:17:07]  finished in 0.146
[01:17:07] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:17:22] 
[01:17:22] running 122 tests
[01:17:46] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:17:51] .i.i......iii.i.....ii
[01:17:51] 
[01:17:51]  finished in 29.321
[01:17:51] travis_fold:end:test_debuginfo

---
[01:27:44]    Compiling core v0.0.0 (/checkout/src/libcore)
[01:27:56] error[E0282]: type annotations needed
[01:27:56]    --> src/libcore/../libcore/tests/result.rs:201:19
[01:27:56]     |
[01:27:56] 201 |         let val = Ok(1)?;
[01:27:56]     |                   |
[01:27:56]     |                   cannot infer type
[01:27:56]     |                   cannot infer type
[01:27:56]     |                   in this expansion of `desugaring of `?``
[01:27:56] 
[01:27:57] error: aborting due to previous error
[01:27:57] 
[01:27:57] For more information about this error, try `rustc --explain E0282`.
[01:27:57] For more information about this error, try `rustc --explain E0282`.
[01:27:57] error: Could not compile `core`.
[01:27:57] 
[01:27:57] To learn more, run the command again with --verbose.
[01:27:57] 
[01:27:57] 
[01:27:57] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:27:57] 
[01:27:57] 
[01:27:57] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:27:57] Build completed unsuccessfully in 0:22:14
[01:27:57] Build completed unsuccessfully in 0:22:14
[01:27:57] make: *** [check] Error 1
[01:27:57] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0247afb9
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon May 13 23:58:51 UTC 2019
---
travis_time:end:171a95b0:start=1557791933459710165,finish=1557791933465856334,duration=6146169
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:095d1615
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01dae156
travis_time:start:01dae156
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:11487a22
$ dmesg | grep -i kill
