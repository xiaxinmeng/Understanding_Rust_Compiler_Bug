plain
travis_time:end:136b7890:start=1561411304925383611,finish=1561411305939634197,duration=1014250586
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
    98% |███████████████████████████████▋| 1.7MB 60.9MB/s eta 0:00:01
    99% |███████████████████████████████▉| 1.7MB 59.9MB/s eta 0:00:01
    100% |████████████████████████████████| 1.7MB 11.6MB/s 
Collecting botocore==1.12.175 (from awscli)
  Downloading https://files.pythonhosted.org/packages/19/ff/fff69109c7f4f97f393b0b948eab16caf3464204fe5cf1955d9d1e1879fa/botocore-1.12.175-py2.py3-none-any.whl (5.6MB)
    0% |▏                               | 20kB 24.8MB/s eta 0:00:01
    0% |▏                               | 30kB 31.1MB/s eta 0:00:01
    0% |▎                               | 40kB 32.8MB/s eta 0:00:01
    0% |▎                               | 51kB 35.3MB/s eta 0:00:01
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:56] 
[01:07:56] running 146 tests
[01:07:59] i..iii......iii..iiii....i............................i..i.....F..F.......i....i.........ii.i.i..i.i 100/146
[01:08:01] i..............i.........iii.i...F.iii........
[01:08:01] 
[01:08:01] ---- [codegen] codegen/issue-45466.rs stdout ----
[01:08:01] 
[01:08:01] 
[01:08:01] error: verification with 'FileCheck' failed
[01:08:01] status: exit code: 1
[01:08:01] command: "/usr/lib/llvm-6.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45466/issue-45466.ll" "/checkout/src/test/codegen/issue-45466.rs"
[01:08:01] ------------------------------------------
[01:08:01] 
[01:08:01] ------------------------------------------
[01:08:01] stderr:
[01:08:01] stderr:
[01:08:01] ------------------------------------------
[01:08:01] /checkout/src/test/codegen/issue-45466.rs:7:11: error: expected string not found in input
[01:08:01] // CHECK: call void @llvm.memset
[01:08:01]           ^
[01:08:01] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45466/issue-45466.ll:14:21: note: scanning from here
[01:08:01] define void @memzero([0 x i8]* nocapture nonnull align 1, i64) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
[01:08:01]                     ^
[01:08:01] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45466/issue-45466.ll:20:2: note: possible intended match here
[01:08:01]  call void @llvm.lifetime.start.p0i8(i64 16, i8* nonnull %2)
[01:08:01] 
[01:08:01] ------------------------------------------
[01:08:01] 
[01:08:01] 
[01:08:01] 
[01:08:01] ---- [codegen] codegen/issue-45222.rs stdout ----
[01:08:01] 
[01:08:01] error: verification with 'FileCheck' failed
[01:08:01] status: exit code: 1
[01:08:01] command: "/usr/lib/llvm-6.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45222/issue-45222.ll" "/checkout/src/test/codegen/issue-45222.rs"
[01:08:01] ------------------------------------------
[01:08:01] 
[01:08:01] ------------------------------------------
[01:08:01] stderr:
[01:08:01] stderr:
[01:08:01] ------------------------------------------
[01:08:01] /checkout/src/test/codegen/issue-45222.rs:23:12: error: expected string not found in input
[01:08:01]  // CHECK: ret i64 500005000000000
[01:08:01]            ^
[01:08:01] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45222/issue-45222.ll:16:23: note: scanning from here
[01:08:01] define i64 @check_foo2() unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
[01:08:01]                       ^
[01:08:01] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45222/issue-45222.ll:29:3: note: possible intended match here
[01:08:01]  store i64 100000, i64* %2, align 8
[01:08:01] /checkout/src/test/codegen/issue-45222.rs:41:12: error: expected string not found in input
[01:08:01]  // CHECK: ret i64 5000050000
[01:08:01]            ^
[01:08:01]            ^
[01:08:01] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45222/issue-45222.ll:163:31: note: scanning from here
[01:08:01] define i64 @check_triangle_inc() unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
[01:08:01]                               ^
[01:08:01] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:08:01] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:08:01] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45222/issue-45222.ll:173:3: note: possible intended match here
[01:08:01]  store i64 100000, i64* %_3.sroa.4.0..sroa_idx10.i, align 8
[01:08:01] /checkout/src/test/codegen/issue-45222.rs:61:12: error: expected string not found in input
[01:08:01]  // CHECK: ret i64 500050000000
[01:08:01]            ^
[01:08:01]            ^
[01:08:01] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45222/issue-45222.ll:258:24: note: scanning from here
[01:08:01] define i64 @check_foo3r() unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
[01:08:01]                        ^
[01:08:01] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45222/issue-45222.ll:271:3: note: possible intended match here
[01:08:01]  store i64 10000, i64* %2, align 8, !noalias !4
[01:08:01] 
[01:08:01] ------------------------------------------
[01:08:01] 
[01:08:01] 
[01:08:01] 
[01:08:01] ---- [codegen] codegen/swap-small-types.rs stdout ----
[01:08:01] 
[01:08:01] error: verification with 'FileCheck' failed
[01:08:01] status: exit code: 1
[01:08:01] command: "/usr/lib/llvm-6.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/swap-small-types/swap-small-types.ll" "/checkout/src/test/codegen/swap-small-types.rs"
[01:08:01] ------------------------------------------
[01:08:01] 
[01:08:01] ------------------------------------------
[01:08:01] stderr:
[01:08:01] stderr:
[01:08:01] ------------------------------------------
[01:08:01] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/swap-small-types/swap-small-types.ll:17:15: error: CHECK-NOT: string occurred!
[01:08:01]  %tmp.i.i.i = alloca %"core::mem::maybe_uninit::MaybeUninit<[u16; 3]>", align 8
[01:08:01]               ^
[01:08:01] /checkout/src/test/codegen/swap-small-types.rs:13:15: note: CHECK-NOT: pattern specified here
[01:08:01] // CHECK-NOT: alloca
[01:08:01] 
[01:08:01] ------------------------------------------
[01:08:01] 
[01:08:01] 
---
[01:08:01] test result: FAILED. 112 passed; 3 failed; 31 ignored; 0 measured; 0 filtered out
[01:08:01] 
[01:08:01] 
[01:08:01] 
[01:08:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:08:01] 
[01:08:01] 
[01:08:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:08:01] Build completed unsuccessfully in 1:03:30
---
travis_time:end:208c8a20:start=1561415400830636043,finish=1561415400835128170,duration=4492127
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00b82e40
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:046ca820
travis_time:start:046ca820
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0fce8330
$ dmesg | grep -i kill
