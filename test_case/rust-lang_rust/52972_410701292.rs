plain
[00:48:46] ....................................................................................................
[00:48:49] ....................................................................................................
[00:48:51] ....................................................................................................
[00:48:54] ....................................................................................................
[00:48:57] iiiiiiiii...........................................................................................
[00:49:03] ....................................................................................................
[00:49:06] .....i..............................................................................................
[00:49:09] ..............i.....................................................................................
[00:49:12] ....................................................................................................
---
[00:56:48] failures:
[00:56:48] 
[00:56:48] ---- [codegen] codegen/vec-clear.rs stdout ----
[00:56:48] 
[00:56:48] error: verification with 'FileCheck' failed
[00:56:48] status: exit code: 1
[00:56:48] command: "/usr/lib/llvm-5.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-clear/vec-clear.ll" "/checkout/src/test/codegen/vec-clear.rs"
[00:56:48] ------------------------------------------
[00:56:48] 
[00:56:48] ------------------------------------------
[00:56:48] stderr:
[00:56:48] stderr:
[00:56:48] ------------------------------------------
[00:56:48] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-clear/vec-clear.ll:18:7: error: CHECK-NOT: string occurred!
[00:56:48]  %1 = load i64, i64* %0, align 8, !range !0, !alias.scope !1
[00:56:48]       ^
[00:56:48] /checkout/src/test/codegen/vec-clear.rs:18:16: note: CHECK-NOT: pattern specified here
[00:56:48]  // CHECK-NOT: load
[00:56:48] 
[00:56:48] ------------------------------------------
[00:56:48] 
[00:56:48] thread '[codegen] codegen/vec-clear.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
---
travis_time:end:22ba34bc:start=1533560768355127014,finish=1533560768361217674,duration=6090660
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:22c51257
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0e306760
travis_time:start:0e306760
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:380f8505
$ dmesg | grep -i kill
