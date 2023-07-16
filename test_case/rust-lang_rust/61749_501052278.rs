plain
travis_time:end:205aa18e:start=1560288898429687048,finish=1560288899253515366,duration=823828318
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
[01:13:32] 
[01:13:32] running 144 tests
[01:13:34] i..iii.....iii..iiii.....i......................i..i............F....i.....i..........ii.i..i..i.ii. 100/144
[01:13:36] .............i.........ii.i..F...iii........
[01:13:36] 
[01:13:36] ---- [codegen] codegen/issue-56927.rs stdout ----
[01:13:36] 
[01:13:36] 
[01:13:36] error: verification with 'FileCheck' failed
[01:13:36] status: exit code: 1
[01:13:36] command: "/usr/lib/llvm-6.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-56927/issue-56927.ll" "/checkout/src/test/codegen/issue-56927.rs"
[01:13:36] ------------------------------------------
[01:13:36] 
[01:13:36] ------------------------------------------
[01:13:36] stderr:
[01:13:36] stderr:
[01:13:36] ------------------------------------------
[01:13:36] /checkout/src/test/codegen/issue-56927.rs:40:11: error: expected string not found in input
[01:13:36] // CHECK: store i32 6, i32* %{{.+}}, align 4
[01:13:36]           ^
[01:13:36] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-56927/issue-56927.ll:96:19: note: scanning from here
[01:13:36] define void @test4(%S* align 16 dereferenceable(16)) unnamed_addr #0 {
[01:13:36]                   ^
[01:13:36] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-56927/issue-56927.ll:114:2: note: possible intended match here
[01:13:36]  store i32 %3, i32* %7, align 4
[01:13:36] 
[01:13:36] ------------------------------------------
[01:13:36] 
[01:13:36] 
[01:13:36] 
[01:13:36] ---- [codegen] codegen/slice-init.rs stdout ----
[01:13:36] 
[01:13:36] error: verification with 'FileCheck' failed
[01:13:36] status: exit code: 1
[01:13:36] command: "/usr/lib/llvm-6.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-init/slice-init.ll" "/checkout/src/test/codegen/slice-init.rs"
[01:13:36] ------------------------------------------
[01:13:36] 
[01:13:36] ------------------------------------------
[01:13:36] stderr:
[01:13:36] stderr:
[01:13:36] ------------------------------------------
[01:13:36] /checkout/src/test/codegen/slice-init.rs:26:12: error: expected string not found in input
[01:13:36]  // CHECK: call void @llvm.memset.p0i8.i[[WIDTH:[0-9]+]](i8* {{.*}}, i8 7, i[[WIDTH]] 4
[01:13:36]            ^
[01:13:36] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-init/slice-init.ll:88:24: note: scanning from here
[01:13:36] define void @byte_array() unnamed_addr #1 {
[01:13:36]                        ^
[01:13:36] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-init/slice-init.ll:95:2: note: possible intended match here
[01:13:36]  call void @llvm.memset.p0i8.i64(i8* %2, i8 %1, i64 4, i32 1, i1 false)
[01:13:36] /checkout/src/test/codegen/slice-init.rs:51:12: error: expected string not found in input
[01:13:36] /checkout/src/test/codegen/slice-init.rs:51:12: error: expected string not found in input
[01:13:36]  // CHECK: call void @llvm.memset.p0i8.i[[WIDTH:[0-9]+]](i8* {{.*}}, i8 0, i[[WIDTH]] 16
[01:13:36]            ^
[01:13:36] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-init/slice-init.ll:128:34: note: scanning from here
[01:13:36] define void @zeroed_integer_array() unnamed_addr #1 {
[01:13:36]                                  ^
[01:13:36] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-init/slice-init.ll:132:2: note: possible intended match here
[01:13:36]  call void @llvm.lifetime.start.p0i8(i64 16, i8* %0)
[01:13:36] 
[01:13:36] ------------------------------------------
[01:13:36] 
[01:13:36] 
---
[01:13:36] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:13:36] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:13:36] 
[01:13:36] 
[01:13:36] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:13:36] 
[01:13:36] 
[01:13:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:36] Build completed unsuccessfully in 1:03:15
---
travis_time:end:03c27a8f:start=1560293364623921289,finish=1560293364681370992,duration=57449703
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0299a8c4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1d645bf8
$ dmesg | grep -i kill
