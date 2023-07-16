plain
travis_time:end:0a8b9aa0:start=1555778769937170543,finish=1555778861195674319,duration=91258503776
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
[01:13:59] 
[01:13:59] running 9 tests
[01:13:59] iiiiiiiii
[01:13:59] 
[01:13:59]  finished in 0.158
[01:13:59] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:14:15] 
[01:14:15] running 121 tests
[01:14:40] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:14:44] i.i......iii.i.....ii
[01:14:44] 
[01:14:44]  finished in 29.577
[01:14:44] travis_fold:end:test_debuginfo

---
[01:39:01]   |
[01:39:01] 1 | extern crate serialize as rustc_serialize;
[01:39:01]   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: convert it to a `use`
[01:39:01]   |
[01:39:01]   = note: `-D unused-extern-crates` implied by `-D rust-2018-idioms`
[01:39:01] error: aborting due to previous error
[01:39:01] 
[01:39:01] error: Could not compile `serialize`.
[01:39:01] warning: build failed, waiting for other jobs to finish...
[01:39:01] warning: build failed, waiting for other jobs to finish...
[01:39:01] error: `extern crate` is not idiomatic in the new edition
[01:39:01]  --> src/libserialize/tests/json.rs:1:1
[01:39:01]   |
[01:39:01] 1 | extern crate serialize as rustc_serialize;
[01:39:01]   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: convert it to a `use`
[01:39:01]   |
[01:39:01]   = note: `-D unused-extern-crates` implied by `-D rust-2018-idioms`
[01:39:01] error: aborting due to previous error
[01:39:01] 
[01:39:01] error: Could not compile `serialize`.
[01:39:01] warning: build failed, waiting for other jobs to finish...
[01:39:01] warning: build failed, waiting for other jobs to finish...
[01:39:05] error: build failed
[01:39:05] 
[01:39:05] 
[01:39:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "serialize" "--" "--quiet"
[01:39:05] 
[01:39:05] 
[01:39:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:39:05] Build completed unsuccessfully in 0:36:54
[01:39:05] Build completed unsuccessfully in 0:36:54
[01:39:05] Makefile:48: recipe for target 'check' failed
[01:39:05] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0047aa40
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Apr 20 18:26:56 UTC 2019
---
travis_time:end:0e26b56b:start=1555784817622832312,finish=1555784817681360345,duration=58528033
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08fbe04e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1f77a708
$ dmesg | grep -i kill
