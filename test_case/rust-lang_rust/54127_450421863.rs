plain
travis_time:end:013fd1ea:start=1546023251318915214,finish=1546023306367351013,duration=55048435799
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
[01:04:41] 
[01:04:41] running 118 tests
[01:05:03] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:05:07] ......iii.i.....ii
[01:05:07] 
[01:05:07]  finished in 26.872
[01:05:07] travis_fold:end:test_debuginfo

---
[01:30:48] 
[01:30:48] failures:
[01:30:48] 
[01:30:48] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0055 (line 1223) stdout ----
[01:30:48] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0055 (line 1223)' panicked at 'test compiled while it wasn't supposed to', src/librustdoc/test.rs:306:13
[01:30:48] 
[01:30:48] 
[01:30:48] failures:
[01:30:48]     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0055 (line 1223)
---
[01:30:48] 
[01:30:48] 
[01:30:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:30:48] Build completed unsuccessfully in 0:36:42
[01:30:48] make: *** [check] Error 1
[01:30:48] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0037df70
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Dec 28 20:26:03 UTC 2018
---
travis_time:end:0b412c00:start=1546028765564399302,finish=1546028765568970015,duration=4570713
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:12cdaa98
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fo
