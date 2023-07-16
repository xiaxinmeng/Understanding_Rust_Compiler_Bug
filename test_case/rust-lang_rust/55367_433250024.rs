plain
travis_time:end:08dee47f:start=1540508236906195230,finish=1540508290845806650,duration=53939611420
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
[00:51:26] .................................................................................................... 2200/4951
[00:51:31] .................................................................................................... 2300/4951
[00:51:34] .................................................................................................... 2400/4951
[00:51:38] .................................................................................................... 2500/4951
[00:51:42] .....................................................iiiiiiiii...................................... 2600/4951
[00:51:49] ...ii............................................................................................... 2800/4951
[00:51:53] .................................................................................................... 2900/4951
[00:51:56] ..............................................................................................i..... 3000/4951
[00:51:59] .................................................................................................... 3100/4951
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:09] 
[01:05:09] running 111 tests
[01:05:12] i..ii...iii.......i...i.........i..iii...........i.....i.....ii...i.i.ii..............i...ii..ii.i.. 100/111
[01:05:12] ..iiii.....
[01:05:12] 
[01:05:12]  finished in 3.610
[01:05:12] travis_fold:end:test_codegen

---
[01:42:07] ---- [ui] rustdoc-ui/private-item-doc-test.rs stdout ----
[01:42:07] 
[01:42:07] error: ui test compiled successfully!
[01:42:07] status: exit code: 0
[01:42:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/private-item-doc-test.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/private-item-doc-test/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/private-item-doc-test/auxiliary"
[01:42:07] ------------------------------------------
[01:42:07] 
[01:42:07] ------------------------------------------
[01:42:07] stderr:
---
[01:42:07] test result: FAILED. 8 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:42:07] 
[01:42:07] 
[01:42:07] 
[01:42:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Zunstable-options " "--target-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:42:07] 
[01:42:07] 
[01:42:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:42:07] Build completed unsuccessfully in 0:55:35
[01:42:07] Build completed unsuccessfully in 0:55:35
[01:42:07] make: *** [check] Error 1
[01:42:07] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2b2d66cc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
