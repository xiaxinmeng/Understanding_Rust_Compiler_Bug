plain
travis_time:end:0ec88bd0:start=1559453678650735843,finish=1559453766830745889,duration=88180010046
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:08:26] 
[01:08:26] running 50 tests
[01:08:43] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:08:43] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:08:43] ......F......................FF...F...............
[01:08:43] 
[01:08:43] ---- [mir-opt] mir-opt/const_prop/const_prop_fails_gracefully.rs stdout ----
[01:08:43] 
[01:08:43] error: compilation failed!
[01:08:43] error: compilation failed!
[01:08:43] status: exit code: 101
[01:08:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/mir-opt/const_prop/const_prop_fails_gracefully.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Zdump-mir=all" "-Zmir-opt-level=3" "-Zdump-mir-exclude-pass-number" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/const_prop_fails_gracefully" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/const_prop_fails_gracefully/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/const_prop_fails_gracefully/auxiliary"
[01:08:43] ------------------------------------------
[01:08:43] 
[01:08:43] ------------------------------------------
[01:08:43] stderr:
[01:08:43] stderr:
[01:08:43] ------------------------------------------
[01:08:43] error: internal compiler error: src/librustc_mir/monomorphize/collector.rs:1205: alloc id without corresponding allocation: 2
[01:08:43] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:637:9
[01:08:43] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:08:43] error: aborting due to previous error
[01:08:43] 
[01:08:43] 
[01:08:43] 
[01:08:43] note: the compiler unexpectedly panicked. this is a bug.
[01:08:43] 
[01:08:43] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:08:43] 
[01:08:43] note: rustc 1.37.0-dev running on x86_64-unknown-linux-gnu
[01:08:43] 
[01:08:43] note: compiler flags: -Z threads=1 -Z dump-mir=all -Z mir-opt-level=3 -Z dump-mir-exclude-pass-number -Z dump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/const_prop_fails_gracefully -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
[01:08:43] 
[01:08:43] ------------------------------------------
[01:08:43] 
[01:08:43] 
[01:08:43] 
[01:08:43] ---- [mir-opt] mir-opt/lower_128bit_debug_test.rs stdout ----
[01:08:43] 
[01:08:43] error: compilation failed!
[01:08:43] status: exit code: 101
[01:08:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/mir-opt/lower_128bit_debug_test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Zdump-mir=all" "-Zmir-opt-level=3" "-Zdump-mir-exclude-pass-number" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/lower_128bit_debug_test" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/lower_128bit_debug_test/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "lower_128bit_ops=yes" "-C" "debug_assertions=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/lower_128bit_debug_test/auxiliary"
[01:08:43] ------------------------------------------
[01:08:43] 
[01:08:43] ------------------------------------------
[01:08:43] stderr:
[01:08:43] stderr:
[01:08:43] ------------------------------------------
[01:08:43] warning: variable does not need to be mutable
[01:08:43]   --> /checkout/src/test/mir-opt/lower_128bit_debug_test.rs:11:23
[01:08:43]    |
[01:08:43] 11 | const fn const_signed(mut x: i128) -> i128 {
[01:08:43]    |                       |
[01:08:43]    |                       help: remove this `mut`
[01:08:43]    |
[01:08:43]    = note: #[warn(unused_mut)] on by default
[01:08:43]    = note: #[warn(unused_mut)] on by default
[01:08:43] 
[01:08:43] warning: variable does not need to be mutable
[01:08:43]   --> /checkout/src/test/mir-opt/lower_128bit_debug_test.rs:15:25
[01:08:43]    |
[01:08:43] 15 | const fn const_unsigned(mut x: u128) -> u128 {
[01:08:43]    |                         |
[01:08:43]    |                         help: remove this `mut`
[01:08:43] 
[01:08:43] thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/libcore/option.rs:347:21
---
[01:08:43] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:08:43] 
[01:08:43] note: rustc 1.37.0-dev running on x86_64-unknown-linux-gnu
[01:08:43] 
[01:08:43] note: compiler flags: -Z threads=1 -Z dump-mir=all -Z mir-opt-level=3 -Z dump-mir-exclude-pass-number -Z dump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/lower_128bit_debug_test -Z unstable-options -Z lower_128bit_ops=yes -C prefer-dynamic -C rpath -C debuginfo=0 -C debug_assertions=yes
[01:08:43] 
[01:08:43] ------------------------------------------
[01:08:43] 
[01:08:43] 
[01:08:43] 
[01:08:43] ---- [mir-opt] mir-opt/lower_128bit_test.rs stdout ----
[01:08:43] 
[01:08:43] error: compilation failed!
[01:08:43] status: exit code: 101
[01:08:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/mir-opt/lower_128bit_test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Zdump-mir=all" "-Zmir-opt-level=3" "-Zdump-mir-exclude-pass-number" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/lower_128bit_test" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/lower_128bit_test/a" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "lower_128bit_ops=yes" "-C" "debug_assertions=no" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/lower_128bit_test/auxiliary"
[01:08:43] ------------------------------------------
[01:08:43] 
[01:08:43] ------------------------------------------
[01:08:43] stderr:
[01:08:43] stderr:
[01:08:43] ------------------------------------------
[01:08:43] warning: variable does not need to be mutable
[01:08:43]  --> /checkout/src/test/mir-opt/lower_128bit_test.rs:8:23
[01:08:43]   |
[01:08:43] 8 | const fn const_signed(mut x: i128) -> i128 {
[01:08:43]   |                       |
[01:08:43]   |                       help: remove this `mut`
[01:08:43]   |
[01:08:43]   = note: #[warn(unused_mut)] on by default
[01:08:43]   = note: #[warn(unused_mut)] on by default
[01:08:43] 
[01:08:43] warning: variable does not need to be mutable
[01:08:43]   --> /checkout/src/test/mir-opt/lower_128bit_test.rs:12:25
[01:08:43]    |
[01:08:43] 12 | const fn const_unsigned(mut x: u128) -> u128 {
[01:08:43]    |                         |
[01:08:43]    |                         help: remove this `mut`
[01:08:43] 
[01:08:43] thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/libcore/option.rs:347:21
---
[01:08:43] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:08:43] 
[01:08:43] note: rustc 1.37.0-dev running on x86_64-unknown-linux-gnu
[01:08:43] 
[01:08:43] note: compiler flags: -Z threads=1 -Z dump-mir=all -Z mir-opt-level=3 -Z dump-mir-exclude-pass-number -Z dump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/lower_128bit_test -Z unstable-options -Z lower_128bit_ops=yes -C prefer-dynamic -C rpath -C debuginfo=0 -C debug_assertions=no
[01:08:43] 
[01:08:43] ------------------------------------------
[01:08:43] 
[01:08:43] 
[01:08:43] 
[01:08:43] ---- [mir-opt] mir-opt/match-arm-scopes.rs stdout ----
[01:08:43] 
[01:08:43] error: compilation failed!
[01:08:43] status: exit code: 101
[01:08:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/mir-opt/match-arm-scopes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Zdump-mir=all" "-Zmir-opt-level=3" "-Zdump-mir-exclude-pass-number" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/match-arm-scopes" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/match-arm-scopes/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/match-arm-scopes/auxiliary"
[01:08:43] ------------------------------------------
[01:08:43] 
[01:08:43] ------------------------------------------
[01:08:43] stderr:
[01:08:43] stderr:
[01:08:43] ------------------------------------------
[01:08:43] warning: unused variable: `s`
[01:08:43]   --> /checkout/src/test/mir-opt/match-arm-scopes.rs:15:20
[01:08:43]    |
[01:08:43] 15 |         (false, a, s) | (a, false, s) if if cond { return 3 } else { a } => 1,
[01:08:43]    |
[01:08:43]    = note: #[warn(unused_variables)] on by default
[01:08:43] help: consider prefixing with an underscore
[01:08:43]    |
[01:08:43]    |
[01:08:43] 15 |         (false, a, _s) | (a, false, _s) if if cond { return 3 } else { a } => 1,
[01:08:43]    |                    ^^               ^^
[01:08:43] warning: unused variable: `b`
[01:08:43]   --> /checkout/src/test/mir-opt/match-arm-scopes.rs:16:16
[01:08:43]    |
[01:08:43]    |
[01:08:43] 16 |         (true, b, t) | (false, b, t) => 2,
[01:08:43] help: consider prefixing with an underscore
[01:08:43]    |
[01:08:43]    |
[01:08:43] 16 |         (true, _b, t) | (false, _b, t) => 2,
[01:08:43]    |                ^^               ^^
[01:08:43] warning: unused variable: `t`
[01:08:43]   --> /checkout/src/test/mir-opt/match-arm-scopes.rs:16:19
[01:08:43]    |
[01:08:43]    |
[01:08:43] 16 |         (true, b, t) | (false, b, t) => 2,
[01:08:43] help: consider prefixing with an underscore
[01:08:43]    |
[01:08:43]    |
[01:08:43] 16 |         (true, b, _t) | (false, b, _t) => 2,
[01:08:43]    |                   ^^               ^^
[01:08:43] 
[01:08:43] error: internal compiler error: src/librustc_codegen_ssa/mir/operand.rs:86: from_const: invalid ByVal layout: TyLayout {
[01:08:43]     ty: [(bool, bool, bool, i32)],
[01:08:43]     details: LayoutDetails {
[01:08:43]         variants: Single {
[01:08:43]             index: 0,
[01:08:43]         fields: Array {
[01:08:43]             stride: Size {
[01:08:43]                 raw: 8,
[01:08:43]             },
[01:08:43]             },
[01:08:43]             count: 0,
[01:08:43]         },
[01:08:43]         abi: Aggregate {
[01:08:43]             sized: false,
[01:08:43]         align: AbiAndPrefAlign {
[01:08:43]             abi: Align {
[01:08:43]                 pow2: 2,
[01:08:43]             },
---
[01:08:43] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:08:43] 
[01:08:43] note: rustc 1.37.0-dev running on x86_64-unknown-linux-gnu
[01:08:43] 
[01:08:43] note: compiler flags: -Z threads=1 -Z dump-mir=all -Z mir-opt-level=3 -Z dump-mir-exclude-pass-number -Z dump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/match-arm-scopes -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
[01:08:43] 
[01:08:43] ------------------------------------------
[01:08:43] 
[01:08:43] 
---
[01:08:43] test result: FAILED. 46 passed; 4 failed; 0 ignored; 0 measured; 0 filtered out
[01:08:43] 
[01:08:43] 
[01:08:43] 
[01:08:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:08:43] 
[01:08:43] 
[01:08:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:08:43] Build completed unsuccessfully in 1:04:00
---
travis_time:end:2091e800:start=1559457901795968697,finish=1559457901865251342,duration=69282645
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01e92a84
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:104cc248
$ dmesg | grep -i kill
