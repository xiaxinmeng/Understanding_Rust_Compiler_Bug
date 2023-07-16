plain
travis_time:end:20ca0f50:start=1544711753415792650,finish=1544711755885989429,duration=2470196779
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:58:19] 
[00:58:19] running 127 tests
[00:58:21] i..ii...iii..iiii.....i...i........F....i...iii............i.....i.........ii...i..i.ii............. 100/127
/test/codegen/inline-always-works-always.NO-OPT/inline-always-works-always.ll:44:5: note: previous match ended here
[00:58:22] bb1: ; preds = %cleanup
[00:58:22]     ^
[00:58:22] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/inline-always-works-always.NO-OPT/inline-always-works-always.ll:45:1: note: non-matching line after previous match is here
[00:58:22]  call void @llvm.trap()
[00:58:22] 
[00:58:22] ------------------------------------------
[00:58:22] 
[00:58:22] 
[00:58:22] thread '[codegen] codegen/inline-always-works-always.rs#NO-OPT' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3294:9
[00:58:22] 
[00:58:22] 
[00:58:22] failures:
[00:58:22] failures:
[00:58:22]     [codegen] codegen/inline-always-works-always.rs#NO-OPT
[00:58:22] test result: FAILED. 92 passed; 1 failed; 34 ignored; 0 measured; 0 filtered out
[00:58:22] 
[00:58:22] 
[00:58:22] 
[00:58:22] 
[00:58:22] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck"ript:"; df -h; du . | sort -nr | head -n100
---
travis_time:end:06207717:start=1544715268742509360,finish=1544715268747431607,duration=4922247
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:089b1a48
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|');
