plain
travis_time:end:110ad200:start=1543421713920084157,finish=1543421769557320876,duration=55637236719
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:47:01] .................................................................................................... 100/5065
[00:47:04] .................................................................................................... 200/5065
[00:47:06] .............................ii............................................ii...................ii.. 300/5065
[00:47:09] ..............................................................................................iii... 400/5065
[00:47:11] .....iiiiiiii.iii............................iii...........................................i........ 500/5065
[00:47:18] .................................................................................................... 700/5065
[00:47:23] ................................................................................................i... 800/5065
[00:47:27] ........i........................................................................................... 900/5065
[00:47:30] ...............iiiii..................ii.iiii....................................................... 1000/5065
---
[00:48:08] .................................................................................................... 2300/5065
[00:48:12] .................................................................................................... 2400/5065
[00:48:16] .................................................................................................... 2500/5065
[00:48:19] .................................................................................................... 2600/5065
[00:48:23] ........iiiiiiiii................................................................................... 2700/5065
[00:48:28] .................................................................................................... 2900/5065
[00:48:32] .................................................................................................... 3000/5065
[00:48:35] .......................................................................i............................ 3100/5065
[00:48:38] .................................................................................................... 3200/5065
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:01:35] 
[01:01:35] running 117 tests
[01:01:38] i..ii...iii..iiii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..i 100/117
[01:01:39] i.i.....iiii.....
[01:01:39] 
[01:01:39]  finished in 3.279
[01:01:39] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:01:52] 
[01:01:52] running 118 tests
[01:02:14] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:02:18] ......iii.i.....ii
[01:02:18] 
[01:02:18]  finished in 25.927
[01:02:18] travis_fold:end:test_debuginfo

---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:29] 
[01:08:29] running 276 tests
[01:09:34] .......................i............................................................F............... 100/276
[01:11:03] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[01:11:03] ............................................................................
[01:11:03] failures:
[01:11:03] 
[01:11:03] 
[01:11:03] ---- [rustdoc] rustdoc/inline_cross/proc_macro.rs stdout ----
[01:11:03] 
[01:11:03] error: htmldocck failed!
[01:11:03] status: exit code: 1
[01:11:03] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/proc_macro" "/checkout/src/test/rustdoc/inline_cross/proc_macro.rs"
[01:11:03] ------------------------------------------
[01:11:03] 
[01:11:03] ------------------------------------------
[01:11:03] stderr:
[01:11:03] stderr:
[01:11:03] ------------------------------------------
[01:11:03] 21: @has check failed
[01:11:03]  `XPATH PATTERN` did not match
[01:11:03]  // @has - '//a/@href' '../some_macros/macro.some_proc_macro.html'
[01:11:03] 23: @has check failed
[01:11:03]  `XPATH PATTERN` did not match
[01:11:03]  // @has - '//a/@href' '../some_macros/derive.SomeDerive.html'
[01:11:03] 24: @!has check failed
[01:11:03]  // @!has proc_macro/macro.some_proc_macro.html
[01:11:03] 26: @!has check failed
[01:11:03]  // @!has proc_macro/derive.SomeDerive.html
[01:11:03] Encountered 4 errors
[01:11:03] 
[01:11:03] ------------------------------------------
[01:11:03] 
---
[01:11:03] test result: FAILED. 273 passed; 1 failed; 2 ignored; 0 measured; 0 filtered out
[01:11:03] 
[01:11:03] 
[01:11:03] 
[01:11:03] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:11:03] 
[01:11:03] 
[01:11:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:11:03] Build completed unsuccessfully in 0:27:46
[01:11:03] Build completed unsuccessfully in 0:27:46
[01:11:03] Makefile:58: recipe for target 'check' failed
[01:11:03] make: *** [check] Error 1
55788 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu
55784 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release
53888 ./.git/modules/src/tools
52160 ./src/llvm/test/CodeGen/X86
---
36560 ./.git/modules/src/libcompiler_builtins/modules
36044 ./.git/modules/src/libcompiler_builtins/modules/compiler-rt
35640 ./src/tools/clang/lib
35580 ./.git/modules/src/libcompiler_builtins/modules/compiler-rt/objects
35572 ./.travis_time:start:0c1d051c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0dfbcfc9
travis_time:start:0dfbcfc9
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:012bba58
$ dmesg | grep -i kill
