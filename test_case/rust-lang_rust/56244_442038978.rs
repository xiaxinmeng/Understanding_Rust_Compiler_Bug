plain
travis_time:end:0bf4e8da:start=1543317329212454364,finish=1543317398077133543,duration=68864679179
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:47:19] .................................................................................................... 100/5064
[00:47:22] .................................................................................................... 200/5064
[00:47:25] .............................ii............................................ii...................ii.. 300/5064
[00:47:27] ..............................................................................................iii... 400/5064
[00:47:30] .....iiiiiiii.iii............................iii...........................................i........ 500/5064
[00:47:37] .................................................................................................... 700/5064
[00:47:42] ................................................................................................i... 800/5064
[00:47:46] ........i........................................................................................... 900/5064
[00:47:49] ...............iiiii..................ii.iiii....................................................... 1000/5064
---
[00:48:26] .................................................................................................... 2300/5064
[00:48:30] .................................................................................................... 2400/5064
[00:48:34] .................................................................................................... 2500/5064
[00:48:37] .................................................................................................... 2600/5064
[00:48:41] .......iiiiiiiii.................................................................................... 2700/5064
[00:48:46] .................................................................................................... 2900/5064
[00:48:50] .................................................................................................... 3000/5064
[00:48:53] .......................................................................i............................ 3100/5064
[00:48:56] .................................................................................................... 3200/5064
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:02:13] 
[01:02:13] running 117 tests
[01:02:16] i..ii...iii...iiii....i...i.........i..iii...........i.....i.....ii...i..i.ii...............i..ii..i 100/117
[01:02:16] i.i......iiii....
[01:02:16] 
[01:02:16]  finished in 3.362
[01:02:16] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:02:30] 
[01:02:30] running 118 tests
[01:02:53] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:02:57] ......iii.i.....ii
[01:02:57] 
[01:02:57]  finished in 27.004
[01:02:57] travis_fold:end:test_debuginfo

---
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:02:57] 
[01:02:57] running 47 tests
[01:03:38] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[01:03:38] .........................................F.....
[01:03:38] 
[01:03:38] ---- [ui] ui-fulldeps/proc-macro/span-preservation.rs stdout ----
[01:03:38] 
[01:03:38] error: /checkout/src/test/ui-fulldeps/proc-macro/span-preservation.rs:1: unexpected error: '1:1: 1:1: mismatched types [E0308]'
[01:03:38] error: /checkout/src/test/ui-fulldeps/proc-macro/span-preservation.rs:1: unexpected error: '1:1: 1:1: mismatched types [E0308]'
[01:03:38] 
[01:03:38] error: /checkout/src/test/ui-fulldeps/proc-macro/span-preservation.rs:12: unexpected error: '12:20: 12:27: mismatched types [E0308]'
[01:03:38] 
[01:03:38] error: /checkout/src/test/ui-fulldeps/proc-macro/span-preservation.rs:18: unexpected error: '18:29: 18:30: mismatched types [E0308]'
[01:03:38] 
[01:03:38] error: /checkout/src/test/ui-fulldeps/proc-macro/span-preservation.rs:34: unexpected error: '34:22: 34:29: mismatched types [E0308]'
[01:03:38] 
[01:03:38] error: /checkout/src/test/ui-fulldeps/proc-macro/span-preservation.rs:35: unexpected error: '35:26: 35:27: struct `c::Foo` has no field named `b` [E0560]'
[01:03:38] error: /checkout/src/test/ui-fulldeps/proc-macro/span-preservation.rs:48: unexpected error: '48:5: 48:6: mismatched types [E0308]'
[01:03:38] 
[01:03:38] error: 6 unexpected errors found, 0 expected errors not found
[01:03:38] status: exit code: 1
[01:03:38] status: exit code: 1
[01:03:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/proc-macro/span-preservation.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/proc-macro/span-preservation/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/proc-macro/span-preservation/auxiliary" "-A" "unused"
[01:03:38]     Error {
[01:03:38]         line_num: 1,
[01:03:38]         kind: Some(
[01:03:38]             Error
---
[01:03:38]         line_num: 35,
[01:03:38]         kind: Some(
[01:03:38]             Error
[01:03:38]         ),
[01:03:38]         msg: "35:26: 35:27: struct `c::Foo` has no field named `b` [E0560]"
[01:03:38]     Error {
[01:03:38]         line_num: 48,
[01:03:38]         kind: Some(
[01:03:38]             Error
---
[01:03:38] test result: FAILED. 46 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:03:38] 
[01:03:38] 
[01:03:38] 
[01:03:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:03:38] 
[01:03:38] 
[01:03:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:03:38] Build completed unsuccessfully in 0:20:03
[01:03:38] Build completed unsuccessfully in 0:20:03
[01:03:38] make: *** [check] Error 1
[01:03:38] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0136df3e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Nov 27 12:20:25 UTC 2018
