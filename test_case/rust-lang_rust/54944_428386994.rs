plain
[00:44:57] .................................................................................................... 2500/4581
[00:45:02] .................................................................................................... 2600/4581
[00:45:05] .................................................................................................... 2700/4581
[00:45:08] .................................................................................................... 2800/4581
[00:45:11] ......................................................................F............................. 2900/4581
[00:45:17] .................................................................................................... 3100/4581
[00:45:20] i.i..ii............................................................................................. 3200/4581
[00:45:24] .................................................................................................... 3300/4581
[00:45:26] ...................................................i................................................ 3400/4581
---
[00:46:03] 
[00:46:03] ---- [ui] ui/lint/must_use-unit.rs stdout ----
[00:46:03] diff of stderr:
[00:46:03] 
[00:46:03] - error: unused return value of `foo` which must be used
[00:46:03] + error: unused return value of `foo` that must be used
[00:46:03] 3    |
[00:46:03] 3    |
[00:46:03] 4 LL |     foo(); //~ unused return value of `foo`
[00:46:03] 
[00:46:03] 10 LL | #![deny(unused_must_use)]
[00:46:03] 12 
[00:46:03] 12 
[00:46:03] - error: unused return value of `bar` which must be used
[00:46:03] + error: unused return value of `bar` that must be used
[00:46:03] checkout/src/test/ui/lint/must_use-unit.rs:16:5\n   |\nLL |     bar(); //~ unused return value of `bar`\n   |     ^^^^^^\n\n"}
[00:46:03] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:46:03] ------------------------------------------
[00:46:03] 
[00:46:03] thread '[ui] ui/lint/must_use-unit.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:46:03] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:46:03] 
[00:46:03] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:499:22
[00:46:03] 
[00:46:03] 
[00:46:03] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options rc/libcompiler_builtins
35584 ./.git/modules/src/libcompiler_builtins/modules
35088 ./.git/modules/src/libcompiler_builtins/modules/compiler-rt
34916 ./obj/build/x86_64-unknown-linux-gnu/native/jemalloc/src
34632 ./.git/modules/src/libcompiler_builtins/modules/compiler-rt/objects
---
travis_time:end:03142bdf:start=1539127637536084621,finish=1539127637539851731,duration=3767110
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1442caa7
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!check
