plain
travis_time:end:1bb41fbe:start=1554416445091454046,finish=1554416446002404593,duration=910950547
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:17:19] 
[01:17:19] running 138 tests
[01:17:22] i..iii.....iii..iiii.....i....................i..i.................i.FF..i..........ii...i..i.ii.... 100/138
[01:17:24] failures:
[01:17:24] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:17:24] 
[01:17:24] ---- [codegen] codegen/mainsubprogram.rs stdout ----
[01:17:24] ---- [codegen] codegen/mainsubprogram.rs stdout ----
[01:17:24] 
[01:17:24] error: verification with 'FileCheck' failed
[01:17:24] status: exit code: 1
[01:17:24] command: "/usr/lib/llvm-6.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/mainsubprogram/mainsubprogram.ll" "/checkout/src/test/codegen/mainsubprogram.rs"
[01:17:24] ------------------------------------------
[01:17:24] 
[01:17:24] ------------------------------------------
[01:17:24] stderr:
[01:17:24] stderr:
[01:17:24] ------------------------------------------
[01:17:24] /checkout/src/test/codegen/mainsubprogram.rs:11:11: error: expected string not found in input
[01:17:24] // CHECK: {{.*}}DISubprogram{{.*}}name: "main",{{.*}}DIFlagMainSubprogram{{.*}}
[01:17:24]           ^
[01:17:24] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/mainsubprogram/mainsubprogram.ll:203:17: note: scanning from here
[01:17:24] define i32 @main(i32, i8**) unnamed_addr #4 {
[01:17:24]                 ^
[01:17:24] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/mainsubprogram/mainsubprogram.ll:246:10: note: possible intended match here
[01:17:24] !17 = distinct !DISubprogram(name: "lang_start<()>", linkageName: "_ZN3std2rt10lang_start17h9f66e6bf3548ca80E", scope: !19, file: !18, line: 61, type: !21, isLocal: true, isDefinition: true, scopeLine: 61, flags: DIFlagPrototyped, isOptimized: true, unit: !14, templateParams: !32, variables: !27)
[01:17:24] 
[01:17:24] ------------------------------------------
[01:17:24] 
[01:17:24] thread '[codegen] codegen/mainsubprogram.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:17:24] thread '[codegen] codegen/mainsubprogram.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:17:24] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:17:24] 
[01:17:24] ---- [codegen] codegen/mainsubprogramstart.rs stdout ----
[01:17:24] 
[01:17:24] error: verification with 'FileCheck' failed
[01:17:24] status: exit code: 1
[01:17:24] command: "/usr/lib/llvm-6.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/mainsubprogramstart/mainsubprogramstart.ll" "/checkout/src/test/codegen/mainsubprogramstart.rs"
[01:17:24] ------------------------------------------
[01:17:24] 
[01:17:24] ------------------------------------------
[01:17:24] stderr:
[01:17:24] stderr:
[01:17:24] ------------------------------------------
[01:17:24] /checkout/src/test/codegen/mainsubprogramstart.rs:10:11: error: expected string not found in input
[01:17:24] // CHECK: {{.*}}DISubprogram{{.*}}name: "start",{{.*}}DIFlagMainSubprogram{{.*}}
[01:17:24]           ^
[01:17:24] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/mainsubprogramstart/mainsubprogramstart.ll:25:17: note: scanning from here
[01:17:24] define i32 @main(i32, i8**) unnamed_addr #2 {
[01:17:24]                 ^
[01:17:24] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/mainsubprogramstart/mainsubprogramstart.ll:48:15: note: possible intended match here
[01:17:24] !6 = distinct !DISubprogram(name: "start", linkageName: "_ZN19mainsubprogramstart5start17h2c6e99a079046318E", scope: !7, file: !4, line: 13, type: !8, isLocal: true, isDefinition: true, scopeLine: 13, flags: DIFlagPrototyped, isOptimized: true, unit: !3, templateParams: !5, variables: !14)
[01:17:24] 
[01:17:24] ------------------------------------------
[01:17:24] 
[01:17:24] thread '[codegen] codegen/mainsubprogramstart.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
---
[01:17:24] test result: FAILED. 107 passed; 2 failed; 29 ignored; 0 measured; 0 filtered out
[01:17:24] 
[01:17:24] 
[01:17:24] 
[01:17:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:17:24] 
[01:17:24] 
[01:17:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:17:24] Build completed unsuccessfully in 0:12:11
[01:17:24] Build completed unsuccessfully in 0:12:11
[01:17:24] Makefile:48: recipe for target 'check' failed
[01:17:24] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03071d54
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Apr  4 23:38:21 UTC 2019
