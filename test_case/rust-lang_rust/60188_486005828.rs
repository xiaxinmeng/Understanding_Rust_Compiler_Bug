plain
travis_time:end:096c56bc:start=1556054514561347462,finish=1556054600170135184,duration=85608787722
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
[01:15:04] 
[01:15:04] running 9 tests
[01:15:04] iiiiiiiii
[01:15:04] 
[01:15:04]  finished in 0.155
[01:15:04] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:15:20] 
[01:15:20] running 121 tests
[01:15:45] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:15:50] i.i......iii.i.....ii
[01:15:50] 
[01:15:50]  finished in 30.175
[01:15:50] travis_fold:end:test_debuginfo

---
[01:41:19] travis_fold:start:test_stage1-syntax
travis_time:start:test_stage1-syntax
Testing syntax stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:41:19]    Compiling syntax v0.0.0 (/checkout/src/libsyntax)
[01:41:30] error[E0063]: missing field `abiguous_block_expr_parse` in initializer of `parse::ParseSess`
[01:41:30]      |
[01:41:30] 1913 |         ParseSess {
[01:41:30] 1913 |         ParseSess {
[01:41:30]      |         ^^^^^^^^^ missing `abiguous_block_expr_parse`
[01:41:32] error: aborting due to previous error
[01:41:32] 
[01:41:32] For more information about this error, try `rustc --explain E0063`.
[01:41:32] error: Could not compile `syntax`.
[01:41:32] error: Could not compile `syntax`.
[01:41:32] 
[01:41:32] To learn more, run the command again with --verbose.
[01:41:32] 
[01:41:32] 
[01:41:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "syntax" "--" "--quiet"
[01:41:32] 
[01:41:32] 
[01:41:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:41:32] Build completed unsuccessfully in 0:38:15
[01:41:32] Build completed unsuccessfully in 0:38:15
[01:41:32] make: *** [check] Error 1
[01:41:32] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:14732058
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Apr 23 23:05:02 UTC 2019
---
travis_time:end:21447960:start=1556060704296408575,finish=1556060704301962862,duration=5554287
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:072737cc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:31064fea
travis_time:start:31064fea
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0fcb1a3d
$ dmesg | grep -i kill
