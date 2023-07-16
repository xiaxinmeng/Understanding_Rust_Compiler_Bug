plain
travis_time:end:2acebe48:start=1560590512425132338,finish=1560590601400697818,duration=88975565480
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
[01:08:16] 
[01:08:16] running 9 tests
[01:08:16] iiiiiiiii
[01:08:16] 
[01:08:16]  finished in 0.159
[01:08:16] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:33] 
[01:08:33] running 122 tests
[01:08:59] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:09:04] .i.i......iii.i.....ii
[01:09:04] 
[01:09:04]  finished in 31.376
[01:09:04] travis_fold:end:test_debuginfo

---
[01:21:15] travis_fold:start:test_stage1-test
travis_time:start:test_stage1-test
Testing test stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:21:16]    Compiling test v0.0.0 (/checkout/src/libtest)
[01:21:16] error: unused import: `TrFailed`
[01:21:16]      |
[01:21:16]      |
[01:21:16] 1779 |         ShouldPanic, StaticTestName, TestDesc, TestDescAndFn, TestOpts, TrFailed, TrFailedMsg,
[01:21:16]      |
[01:21:16]      = note: `-D unused-imports` implied by `-D warnings`
[01:21:16] 
[01:21:16] error: aborting due to previous error
[01:21:16] error: aborting due to previous error
[01:21:16] 
[01:21:16] error: Could not compile `test`.
[01:21:16] 
[01:21:16] To learn more, run the command again with --verbose.
[01:21:16] 
[01:21:16] 
[01:21:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "-p" "test" "--" "--quiet"
[01:21:16] 
[01:21:16] 
[01:21:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:21:16] Build completed unsuccessfully in 1:17:10
---
travis_time:end:083106a0:start=1560595489238358081,finish=1560595489244827888,duration=6469807
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03c5ee84
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
