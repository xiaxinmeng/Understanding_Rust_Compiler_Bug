plain
travis_time:end:0284d138:start=1541726249800454155,finish=1541726250841274796,duration=1040820641
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:55:02] .................................................................................................... 100/4999
[00:55:05] .................................................................................................... 200/4999
[00:55:08] ........................................................................ii...................ii..... 300/4999
[00:55:11] ...........................................................................................iii...... 400/4999
[00:55:14] ..iiiiiiii.iii...........................iii...........................................i...........i 500/4999
[00:55:21] .................................................................................................... 700/4999
[00:55:28] .....................................................................i...........i.................. 800/4999
[00:55:31] ........................................................................................iiiii....... 900/4999
[00:55:34] ...........ii.iiii.................................................................................. 1000/4999
---
[00:56:11] .................................................................................................... 2200/4999
[00:56:16] .................................................................................................... 2300/4999
[00:56:20] .................................................................................................... 2400/4999
[00:56:24] .................................................................................................... 2500/4999
[00:56:28] ...................................................................iiiiiiiii........................ 2600/4999
[00:56:35] ...............................ii................................................................... 2800/4999
[00:56:38] .................................................................................................... 2900/4999
[00:56:42] .................................................................................................... 3000/4999
[00:56:45] ..........................i......................................................................... 3100/4999
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:11:32] 
[01:11:32] running 115 tests
[01:11:35] i..ii...iii..iii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..ii 100/115
[01:11:36] .i....iiii.....
[01:11:36] 
[01:11:36]  finished in 3.732
[01:11:36] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:11:51] 
[01:11:51] running 118 tests
[01:12:15] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:12:19] ......iii.i.....ii
[01:12:19] 
[01:12:19]  finished in 28.634
[01:12:19] travis_fold:end:test_debuginfo

---
[01:19:32] 
[01:19:32] running 274 tests
[01:20:55] .......................i............................................................................ 100/274
[01:22:02] ................................i................................................................... 200/274
[01:22:55] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[01:22:55] ............................................F.............................
[01:22:55] 
[01:22:55] ---- [rustdoc] rustdoc/src-links-external.rs stdout ----
[01:22:55] 
[01:22:55] 
[01:22:55] error: htmldocck failed!
[01:22:55] status: exit code: 1
[01:22:55] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/src-links-external" "/checkout/src/test/rustdoc/src-links-external.rs"
[01:22:55] ------------------------------------------
[01:22:55] 
[01:22:55] ------------------------------------------
[01:22:55] stderr:
[01:22:55] stderr:
[01:22:55] ------------------------------------------
[01:22:55] 20: @has check failed
[01:22:55]  File does not exist 'foo/bar/index.html'
[01:22:55]  // @has foo/bar/index.html '//a/@href' '../../src/src_links_external/src-links-external.rs.html#11'
[01:22:55] 23: @has check failed
[01:22:55]  File does not exist 'foo/bar/struct.Foo.html'
[01:22:55]  // @has foo/bar/struct.Foo.html '//a/@href' '../../src/src_links_external/src-links-external.rs.html#11'
[01:22:55] Encountered 2 errors
[01:22:55] 
[01:22:55] ------------------------------------------
[01:22:55] 
---
[01:22:55] test result: FAILED. 271 passed; 1 failed; 2 ignored; 0 measured; 0 filtered out
[01:22:55] 
[01:22:55] 
[01:22:55] 
[01:22:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:22:55] 
[01:22:55] 
[01:22:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:22:55] Build completed unsuccessfully in 0:31:52
[01:22:55] Build completed unsuccessfully in 0:31:52
[01:22:55] make: *** [check] Error 1
[01:22:55] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:125d3ee5
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Nov  9 02:40:38 UTC 2018
