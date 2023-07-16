plain
travis_time:end:0f6de56a:start=1545960519757192942,finish=1545960599037553785,duration=79280360843
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
[01:09:31] 
[01:09:31] running 118 tests
[01:09:55] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:09:59] ......iii.i.....ii
[01:09:59] 
[01:09:59]  finished in 29.037
[01:09:59] travis_fold:end:test_debuginfo

---
[01:15:20] 
[01:15:20] running 287 tests
[01:16:28] ..........................i......................................................................... 100/287
[01:17:26] .....................................i.............................................................. 200/287
16] 30: @has check failed
[01:18:16]  `XPATH PATTERN` did not match
[01:18:16]  // @has - '//*[@id="synthetic-implementations-list"]/*[@class="impl"]//*/code' "impl<'a> Send for Parser<'a>"
[01:18:16] Encountered 1 errors
[01:18:16] 
[01:18:16] ------------------------------------------
[01:18:16] 
---
[01:18:16] 
[01:18:16] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:18:16] 
[01:18:16] 
[01:18:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/ch
