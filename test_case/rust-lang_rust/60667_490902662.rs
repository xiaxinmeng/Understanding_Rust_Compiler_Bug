plain
travis_time:end:0aac4648:start=1557402614815030543,finish=1557402709621061353,duration=94806030810
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
[01:20:40] 
[01:20:40] running 143 tests
[01:20:42] i..iii.....iii..iiii.....i......................i..i.................i.....i..........ii..i.i..i.ii. 100/143
[01:20:44] test result: ok. 113 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:20:44] 
[01:20:44]  finished in 4.653
[01:20:44] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:20:46] 
[01:20:46] running 9 tests
[01:20:46] iiiiiiiii
[01:20:46] 
[01:20:46]  finished in 0.150
[01:20:46] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:21:02] 
[01:21:02] running 122 tests
[01:21:27] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:21:32] .i.i......iii.i.....ii
[01:21:32] 
[01:21:32]  finished in 29.882
[01:21:32] travis_fold:end:test_debuginfo

---
[01:31:51] 
[01:31:51] To learn more, run the command again with --verbose.
[01:31:51] 
[01:31:51] 
[01:31:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:31:51] 
[01:31:51] 
[01:31:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:31:51] Build completed unsuccessfully in 0:22:52
[01:31:51] Build completed unsuccessfully in 0:22:52
[01:31:51] make: *** [check] Error 1
[01:31:51] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06db6ef0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu May  9 13:23:50 UTC 2019
