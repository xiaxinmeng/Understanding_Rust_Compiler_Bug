plain
travis_time:end:00e270c0:start=1560023211176478861,finish=1560023300585420235,duration=89408941374
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:54:33] .................................................................................................... 400/5657
[00:54:36] .................................................................................................... 500/5657
[00:54:40] .........................i.......................................................................... 600/5657
[00:54:44] .................................................................................................... 700/5657
[00:54:49] ..........................................................F.............................F........... 800/5657
[00:54:58] .......................i...........i................................................................ 1000/5657
[00:55:01] ....................................................iiiii........................................... 1100/5657
[00:55:05] .................................................................................................... 1200/5657
[00:55:07] .................................................................................................... 1300/5657
---
[00:57:56] 
[00:57:56] ---- [ui] ui/consts/const-eval/const_raw_ptr_ops.rs stdout ----
[00:57:56] diff of stderr:
[00:57:56] 
[00:57:56] 14 LL | const Y2: usize = unsafe { &1 as *const i32 as usize + 1 };
[00:57:56] 15    | ---------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
[00:57:56] 16    |                            |
[00:57:56] -    |                            "pointer arithmetic or comparison" needs an rfc before being allowed inside constants
[00:57:56] 18 
[00:57:56] 19 error: any use of this value will cause an error
[00:57:56] 20   --> $DIR/const_raw_ptr_ops.rs:16:26
[00:57:56] 
[00:57:56] 
[00:57:56] 
[00:57:56] The actual stderr differed from the expected stderr.
[00:57:56] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_raw_ptr_ops/const_raw_ptr_ops.stderr
[00:57:56] To update references, rerun the tests and pass the `--bless` flag
[00:57:56] To only update this specific test, also pass `--test-args consts/const-eval/const_raw_ptr_ops.rs`
[00:57:56] error: 1 errors occurred comparing output.
[00:57:56] status: exit code: 1
[00:57:56] status: exit code: 1
[00:57:56] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_raw_ptr_ops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_raw_ptr_ops" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_raw_ptr_ops/auxiliary" "-A" "unused"
[00:57:56] ------------------------------------------
[00:57:56] 
[00:57:56] ------------------------------------------
[00:57:56] stderr:
[00:57:56] stderr:
[00:57:56] ------------------------------------------
[00:57:56] error: any use of this value will cause an error
[00:57:56]   --> /checkout/src/test/ui/consts/const-eval/const_raw_ptr_ops.rs:6:26
[00:57:56]    |
[00:57:56] LL | const X: bool = unsafe { &1 as *const i32 == &2 as *const i32 }; //~ ERROR any use of this
[00:57:56]    | -------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
[00:57:56]    |                          |
[00:57:56]    |                          "pointer arithmetic or comparison" needs an rfc before being allowed inside constants
[00:57:56]    = note: #[deny(const_err)] on by default
[00:57:56] 
[00:57:56] error: any use of this value will cause an error
[00:57:56]   --> /checkout/src/test/ui/consts/const-eval/const_raw_ptr_ops.rs:12:28
[00:57:56]   --> /checkout/src/test/ui/consts/const-eval/const_raw_ptr_ops.rs:12:28
[00:57:56]    |
[00:57:56] LL | const Y2: usize = unsafe { &1 as *const i32 as usize + 1 }; //~ ERROR any use of this
[00:57:56]    | ---------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
[00:57:56]    |                            a raw memory access tried to access part of a pointer value as raw bytes
[00:57:56] 
[00:57:56] error: any use of this value will cause an error
[00:57:56]   --> /checkout/src/test/ui/consts/const-eval/const_raw_ptr_ops.rs:16:26
[00:57:56]   --> /checkout/src/test/ui/consts/const-eval/const_raw_ptr_ops.rs:16:26
[00:57:56]    |
[00:57:56] LL | const Z2: i32 = unsafe { *(42 as *const i32) }; //~ ERROR any use of this value will cause
[00:57:56]    | -------------------------^^^^^^^^^^^^^^^^^^^---
[00:57:56]    |                          a memory access tried to interpret some bytes as a pointer
[00:57:56] 
[00:57:56] error: any use of this value will cause an error
[00:57:56]   --> /checkout/src/test/ui/consts/const-eval/const_raw_ptr_ops.rs:17:26
[00:57:56]   --> /checkout/src/test/ui/consts/const-eval/const_raw_ptr_ops.rs:17:26
[00:57:56]    |
[00:57:56] LL | const Z3: i32 = unsafe { *(44 as *const i32) }; //~ ERROR any use of this value will cause
[00:57:56]    | -------------------------^^^^^^^^^^^^^^^^^^^---
[00:57:56]    |                          a memory access tried to interpret some bytes as a pointer
[00:57:56] 
[00:57:56] error: aborting due to 4 previous errors
[00:57:56] 
---
[00:57:56] 
[00:57:56] 17   --> $DIR/match-test-ptr-null.rs:9:13
[00:57:56] 18    |
[00:57:56] 19 LL |             0 => 42,
[00:57:56] -    |             ^ "pointer arithmetic or comparison" needs an rfc before being allowed inside constants
[00:57:56] +    |             ^ a raw memory access tried to access part of a pointer value as raw bytes
[00:57:56] 22 error: aborting due to 3 previous errors
[00:57:56] 23 
[00:57:56] 
[00:57:56] 
[00:57:56] 
[00:57:56] The actual stderr differed from the expected stderr.
[00:57:56] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/match-test-ptr-null/match-test-ptr-null.stderr
[00:57:56] To update references, rerun the tests and pass the `--bless` flag
[00:57:56] To only update this specific test, also pass `--test-args consts/const-eval/match-test-ptr-null.rs`
[00:57:56] error: 1 errors occurred comparing output.
[00:57:56] status: exit code: 1
[00:57:56] status: exit code: 1
[00:57:56] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/match-test-ptr-null.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/match-test-ptr-null" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/match-test-ptr-null/auxiliary" "-A" "unused"
[00:57:56] ------------------------------------------
[00:57:56] 
[00:57:56] ------------------------------------------
[00:57:56] stderr:
[00:57:56] stderr:
[00:57:56] ------------------------------------------
[00:57:56] error[E0658]: casting pointers to integers in constants is unstable
[00:57:56]   --> /checkout/src/test/ui/consts/const-eval/match-test-ptr-null.rs:6:15
[00:57:56]    |
[00:57:56] LL |         match &1 as *const i32 as usize {
[00:57:56]    |
[00:57:56]    = note: for more information, see https://github.com/rust-lang/rust/issues/51910
[00:57:56]    = help: add #![feature(const_raw_ptr_to_usize_cast)] to the crate attributes to enable
[00:57:56] 
[00:57:56] 
[00:57:56] error[E0019]: constant contains unimplemented expression type
[00:57:56]   --> /checkout/src/test/ui/consts/const-eval/match-test-ptr-null.rs:9:13
[00:57:56]    |
[00:57:56] LL |             0 => 42, //~ ERROR constant contains unimplemented expression type
[00:57:56] 
[00:57:56] error[E0080]: evaluation of constant value failed
[00:57:56]   --> /checkout/src/test/ui/consts/const-eval/match-test-ptr-null.rs:9:13
[00:57:56]    |
[00:57:56]    |
[00:57:56] LL |             0 => 42, //~ ERROR constant contains unimplemented expression type
[00:57:56]    |             ^ a raw memory access tried to access part of a pointer value as raw bytes
[00:57:56] error: aborting due to 3 previous errors
[00:57:56] 
[00:57:56] Some errors have detailed explanations: E0019, E0080, E0658.
[00:57:56] For more information about an error, try `rustc --explain E0019`.
---
[00:57:56] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[00:57:56] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:57:56] 
[00:57:56] 
[00:57:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:57:56] 
[00:57:56] 
[00:57:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:57:56] Build completed unsuccessfully in 0:54:03
---
travis_time:end:07f1a090:start=1560026787056548214,finish=1560026787063975388,duration=7427174
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:13a9c0a0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:11c4c842
$ dmesg | grep -i kill
