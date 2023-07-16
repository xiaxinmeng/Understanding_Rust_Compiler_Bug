plain
travis_time:end:0d001268:start=1544961011779114112,finish=1544961067050664337,duration=55271550225
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:51:17] 
[00:51:17] running 121 tests
[00:51:20] i..ii...iii..iiii.....i...i..........i..iii.............i.....i......ii...i..i.ii..............i...i 100/121
[00:51:20] i..ii..i....iiii.....
[00:51:20] 
[00:51:20]  finished in 3.384
[00:51:20] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:51:34] 
[00:51:34] running 119 tests
[00:51:56] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii.........i.i...ii...i.......ii.i.i. 100/119
[00:52:00] i......iii.i.....ii
[00:52:00] 
[00:52:00]  finished in 25.812
[00:52:00] travis_fold:end:test_debuginfo

---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:56:50] 
[00:56:50] running 281 tests
[00:57:57] .F..............F......i............................................................................ 100/281
[00:58:52] .................................i.F........................................................F....F.F 200/281
[00:59:37] stdout:
[00:59:37] ------------------------------------------
[00:59:37] 
[00:59:37] ------------------------------------------
[00:59:37] ------------------------------------------
[00:59:37] stderr:
[00:59:37] ------------------------------------------
[00:59:37] 21: @has check failed
[00:59:37]  `XPATH PATTERN` did not match
[00:59:37]  // @has - '//*[@id="synthetic-implementations-list"]/*[@class="impl"]//*/code' "impl<'c, K> Send for Foo<'c, K> where K: for<'b> Fn(&'b bool) -> &'c u8, 'c: 'static"
[00:59:37] 24: @has check failed
[00:59:37]  `XPATH PATTERN` did not match
[00:59:37]  // @has - '//*[@id="synthetic-implementations-list"]/*[@class="impl"]//*/code' "impl<'c, K> Sync for Foo<'c, K> where K: Sync"
[00:59:37] Encountered 2 errors
[00:59:37] 
[00:59:37] ------------------------------------------
[00:59:37] 
[00:59:37] 
[00:59:37] thread '[rustdoc] rustdoc/synthetic_auto/lifetimes.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:59:37] 
[00:59:37] ---- [rustdoc] rustdoc/synthetic_auto/manual.rs stdout ----
[00:59:37] 
[00:59:37] error: htmldocck failed!
[00:59:37] status: exit code: 1
[00:59:37] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/manual" "/checkout/src/test/rustdoc/synthetic_auto/manual.rs"
[00:59:37] ------------------------------------------
[00:59:37] 
[00:59:37] ------------------------------------------
[00:59:37] stderr:
[00:59:37] stderr:
[00:59:37] ------------------------------------------
[00:59:37] 12: @has check failed
[00:59:37]  `XPATH PATTERN` did noout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:59:37] 
[00:59:37] 
[00:59:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:59:37] Build completed unsuccessfully in 0:18:29
[00:59:37] Build completed unsuccessfully in 0:18:29
[00:59:37] make: *** [check] Error 1
[00:59:37] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0010b3a6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Dec 16 12:50:53 UTC 2018
