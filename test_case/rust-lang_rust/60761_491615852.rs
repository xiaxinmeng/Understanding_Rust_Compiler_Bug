plain
travis_time:end:0fd9c158:start=1557678234532565164,finish=1557678235291754463,duration=759189299
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
[01:18:48] 
[01:18:48] running 143 tests
[01:18:51] i..iii.....iii..iiii.....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/143
[01:18:53] test result: ok. 113 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:18:53] 
[01:18:53]  finished in 4.641
[01:18:53] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:18:55] 
[01:18:55] running 9 tests
[01:18:55] iiiiiiiii
[01:18:55] 
[01:18:55]  finished in 0.150
[01:18:55] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:10] 
[01:19:10] running 122 tests
[01:19:34] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:19:39] .i.i......iii.i.....ii
[01:19:39] 
[01:19:39]  finished in 28.844
[01:19:39] travis_fold:end:test_debuginfo

---
[01:32:42] running 922 tests
[01:33:02] i................................................................................................... 100/922
[01:33:12] .................................iii......i......i...i......i....................................... 200/922
[01:33:16] .................................................................................................... 300/922
[01:33:23] .................................................i.i....................F...................iiii.... 400/922
[01:33:37] .................................................................................................... 600/922
[01:33:45] ............................................iiii.................................................... 700/922
[01:33:59] .................................................................................................... 800/922
[01:34:06] ....................................................................iiii............................ 900/922
---
[01:34:07]  --> keyword_docs.rs:567:9
[01:34:07]   |
[01:34:07] 1 | #![deny(warnings)]
[01:34:07]   |         ^^^^^^^^
[01:34:07]   = note: #[deny(while_true)] implied by #[deny(warnings)]
[01:34:07] error: aborting due to previous error
[01:34:07] 
[01:34:07] 
[01:34:07] thread 'keyword_docs.rs - while_keyword (line 569)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:319:13
[01:34:07] 
[01:34:07] 
[01:34:07] failures:
[01:34:07]     keyword_docs.rs - while_keyword (line 569)
[01:34:07]     keyword_docs.rs - while_keyword (line 569)
[01:34:07] 
[01:34:07] test result: FAILED. 897 passed; 1 failed; 24 ignored; 0 measured; 0 filtered out
[01:34:07] 
[01:34:07] error: test failed, to rerun pass '--doc'
[01:34:07] 
[01:34:07] 
[01:34:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:34:07] 
[01:34:07] 
[01:34:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:34:07] Build completed unsuccessfully in 0:26:56
[01:34:07] Build completed unsuccessfully in 0:26:56
[01:34:07] make: *** [check] Error 1
[01:34:07] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:07e86172
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun May 12 17:58:13 UTC 2019
---
travis_time:end:02855c86:start=1557683895402201899,finish=1557683895453525527,duration=51323628
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1e538a19
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2d09cd0a
$ dmesg | grep -i kill
