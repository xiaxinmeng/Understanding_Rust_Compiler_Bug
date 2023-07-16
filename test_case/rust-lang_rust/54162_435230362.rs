plain
travis_time:end:0d1e8e65:start=1541113136809346707,finish=1541113192063172597,duration=55253825890
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:53:53] .................................................................................................... 100/4983
[00:53:56] .................................................................................................... 200/4983
[00:53:59] ...........................................................................................ii....... 300/4983
[00:54:02] .........................................................................................iii........ 400/4983
[00:54:05] iiiiiiii.iii...........................iii...........................................i...........i.. 500/4983
[00:54:12] .................................................................................................... 700/4983
[00:54:19] ..................................................................i...........i..................... 800/4983
[00:54:22] ....................................................................................iiiii........... 900/4983
[00:54:26] .................................................................................................... 1000/4983
---
[00:55:04] .................................................................................................... 2200/4983
[00:55:09] .................................................................................................... 2300/4983
[00:55:13] .................................................................................................... 2400/4983
[00:55:17] .................................................................................................... 2500/4983
[00:55:21] ...................................................................iiiiiiiii........................ 2600/4983
[00:55:29] ..................ii................................................................................ 2800/4983
[00:55:32] .................................................................................................... 2900/4983
[00:55:36] .................................................................................................... 3000/4983
[00:55:39] .............i...................................................................................... 3100/4983
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:10:14] 
[01:10:14] running 115 tests
[01:10:17] i..ii...iii..iii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..ii 100/115
[01:10:18] .i....iiii.....
[01:10:18] 
[01:10:18]  finished in 3.710
[01:10:18] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:10:33] 
[01:10:33] running 118 tests
[01:10:58] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:11:02] ......iii.i.....ii
[01:11:02] 
[01:11:02]  finished in 28.928
[01:11:02] travis_fold:end:test_debuginfo

---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:17:18] 
[01:17:18] running 270 tests
[01:18:28] ...F...................i............................................................................ 100/270
[01:20:09] ......................................................................
[01:20:09] failures:
[01:20:09] 
[01:20:09] ---- [rustdoc] rustdoc/assoc-consts.rs stdout ----
[01:20:09] ---- [rustdoc] rustdoc/assoc-consts.rs stdout ----
[01:20:09] 
[01:20:09] error: htmldocck failed!
[01:20:09] status: exit code: 1
[01:20:09] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-consts" "/checkout/src/test/rustdoc/assoc-consts.rs"
[01:20:09] ------------------------------------------
[01:20:09] 
[01:20:09] ------------------------------------------
[01:20:09] stderr:
[01:20:09] stderr:
[01:20:09] ------------------------------------------
[01:20:09] 102: @has check failed
[01:20:09]  `XPATH PATTERN` did not match
[01:20:09]      // @has - '//*[@class="docblock"]' "Docs for QUX_DEFAULT12 in trait."
[01:20:09] Encountered 1 errors
[01:20:09] 
[01:20:09] ------------------------------------------
[01:20:09] 
---
[01:20:09] 
[01:20:09] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[01:20:09] 
[01:20:09] 
[01:20:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:20:09] 
[01:20:09] 
[01:20:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:20:09] Build completed unsuccessfully in 0:30:13
[01:20:09] Build completed unsuccessfully in 0:30:13
[01:20:09] make: *** [check] Error 1
[01:20:09] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1a144aa7
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
