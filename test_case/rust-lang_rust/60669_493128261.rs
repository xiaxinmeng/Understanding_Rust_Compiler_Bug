plain
travis_time:end:0792f2cd:start=1558015988544279667,finish=1558015991519261385,duration=2974981718
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
[01:25:14] 
[01:25:14] running 143 tests
[01:25:17] i..iii.....iii...iiii....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/143
[01:25:19] test result: ok. 113 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:25:19] 
[01:25:19]  finished in 5.104
[01:25:19] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:25:22] 
[01:25:22] running 9 tests
[01:25:22] iiiiiiiii
[01:25:22] 
[01:25:22]  finished in 0.175
[01:25:22] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:25:39] 
[01:25:39] running 122 tests
[01:26:06] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:26:11] .i.i......iii.i.....ii
[01:26:11] 
[01:26:11]  finished in 32.288
[01:26:11] travis_fold:end:test_debuginfo

---
[01:45:17] travis_fold:start:test_stage1-syntax
travis_time:start:test_stage1-syntax
Testing syntax stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:45:18]    Compiling syntax v0.0.0 (/checkout/src/libsyntax)
[01:45:28] error[E0063]: missing field `param_attr_spans` in initializer of `parse::ParseSess`
[01:45:28]      |
[01:45:28] 1574 |         ParseSess {
[01:45:28]      |         ^^^^^^^^^ missing `param_attr_spans`
[01:45:28] 
---
[01:45:30] 
[01:45:30] To learn more, run the command again with --verbose.
[01:45:30] 
[01:45:30] 
[01:45:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "syntax" "--" "--quiet"
[01:45:30] 
[01:45:30] 
[01:45:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:45:30] Build completed unsuccessfully in 0:33:18
[01:45:30] Build completed unsuccessfully in 0:33:18
[01:45:30] make: *** [check] Error 1
[01:45:30] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02fc4741
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu May 16 15:58:52 UTC 2019
---
travis_time:end:098eecfe:start=1558022334214518417,finish=1558022334219393655,duration=4875238
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:33a4cd78
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0d202f62
travis_time:start:0d202f62
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2e4336df
$ dmesg | grep -i kill
