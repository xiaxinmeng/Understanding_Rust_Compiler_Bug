plain
travis_time:end:05006adb:start=1550857689600016650,finish=1550857691740919854,duration=2140903204
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_incremental
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:09:27] 
[01:09:27] running 98 tests
[01:09:40] ..............................F.....................F.....F.....F..F.............F..F.F...........
[01:09:40] 
[01:09:40] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:09:40] ---- [incremental] incremental/hashes/for_loops.rs stdout ----
[01:09:40] 
[01:09:40] 
[01:09:40] error in revision `cfail3`: test compilation failed although it shouldn't!
[01:09:40] status: exit code: 101
[01:09:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/for_loops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/for_loops.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/auxiliary"
[01:09:40] ------------------------------------------
[01:09:40] 
[01:09:40] ------------------------------------------
[01:09:40] stderr:
[01:09:40] stderr:
[01:09:40] ------------------------------------------
[01:09:40] thread 'rustc' panicked at 'assertion failed: pos < self.opaque.data().len()', src/librustc/ty/query/on_disk_cache.rs:571:9
[01:09:40] 
[01:09:40] error: internal compiler error: unexpected panic
[01:09:40] 
[01:09:40] note: the compiler unexpectedly panicked. this is a bug.
[01:09:40] note: the compiler unexpectedly panicked. this is a bug.
[01:09:40] 
[01:09:40] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:09:40] 
[01:09:40] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:09:40] 
[01:09:40] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[01:09:40] 
[01:09:40] ------------------------------------------
[01:09:40] 
[01:09:40] thread '[incremental] incremental/hashes/for_loops.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:09:40] thread '[incremental] incremental/hashes/for_loops.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:09:40] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:09:40] 
[01:09:40] ---- [incremental] incremental/incremental_proc_macro.rs stdout ----
[01:09:40] 
[01:09:40] error in revision `cfail2`: auxiliary build of "/checkout/src/test/incremental/auxiliary/incremental_proc_macro_aux.rs" failed to compile: 
[01:09:40] status: exit code: 101
[01:09:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/auxiliary/incremental_proc_macro_aux.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/incremental_proc_macro/incremental_proc_macro.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/incremental_proc_macro/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/incremental_proc_macro/auxiliary"
[01:09:40] ------------------------------------------
[01:09:40] 
[01:09:40] ------------------------------------------
[01:09:40] stderr:
[01:09:40] stderr:
[01:09:40] ------------------------------------------
[01:09:40] {"message":"unused variable: `input`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/incremental/auxiliary/incremental_proc_macro_aux.rs","byte_start":262,"byte_end":267,"line_start":15,"line_end":15,"column_start":15,"column_end":20,"is_primary":true,"text":[{"text":"pub fn derive(input: TokenStream) -> TokenStream {","highlight_start":15,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_variables)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider prefixing with an underscore","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/incremental/auxiliary/incremental_proc_macro_aux.rs","byte_start":262,"byte_end":267,"line_start":15,"line_end":15,"column_start":15,"column_end":20,"is_primary":true,"text":[{"text":"pub fn derive(input: TokenStream) -> TokenStream {","highlight_start":15,"highlight_end":20}],"label":null,"suggested_replacement":"_input","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `input`\n  --> /checkout/src/test/incremental/auxiliary/incremental_proc_macro_aux.rs:15:15\n   |\nLL | pub fn derive(input: TokenStream) -> TokenStream {\n   |               ^^^^^ help: consider prefixing with an underscore: `_input`\n   |\n   = note: #[warn(unused_variables)] on by default\n\n"}
[01:09:40] thread 'rustc' panicked at 'assertion failed: pos < self.opaque.data().len()', src/librustc/ty/query/on_disk_cache.rs:571:9
[01:09:40] 
[01:09:40] error: internal compiler error: unexpected panic
[01:09:40] 
[01:09:40] note: the compiler unexpectedly panicked. this is a bug.
[01:09:40] note: the compiler unexpectedly panicked. this is a bug.
[01:09:40] 
[01:09:40] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:09:40] 
[01:09:40] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:09:40] 
[01:09:40] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C rpath
[01:09:40] 
[01:09:40] ------------------------------------------
[01:09:40] 
[01:09:40] thread '[incremental] incremental/incremental_proc_macro.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:09:40] thread '[incremental] incremental/incremental_proc_macro.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:09:40] 
[01:09:40] ---- [incremental] incremental/issue-49595/issue_49595.rs stdout ----
[01:09:40] 
[01:09:40] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:09:40] status: exit code: 101
[01:09:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-49595/issue_49595.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-49595/issue_49595/issue_49595.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-49595/issue_49595/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-49595/issue_49595/auxiliary"
[01:09:40] ------------------------------------------
[01:09:40] 
[01:09:40] ------------------------------------------
[01:09:40] stderr:
[01:09:40] stderr:
[01:09:40] ------------------------------------------
[01:09:40] thread 'rustc' panicked at 'assertion failed: pos < self.opaque.data().len()', src/librustc/ty/query/on_disk_cache.rs:571:9
[01:09:40] 
[01:09:40] error: internal compiler error: unexpected panic
[01:09:40] 
[01:09:40] note: the compiler unexpectedly panicked. this is a bug.
[01:09:40] note: the compiler unexpectedly panicked. this is a bug.
[01:09:40] 
[01:09:40] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:09:40] 
[01:09:40] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:09:40] 
[01:09:40] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:09:40] 
[01:09:40] ------------------------------------------
[01:09:40] 
[01:09:40] thread '[incremental] incremental/issue-49595/issue_49595.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:09:40] thread '[incremental] incremental/issue-49595/issue_49595.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:09:40] 
[01:09:40] ---- [incremental] incremental/krate-inlined.rs stdout ----
[01:09:40] 
[01:09:40] error in revision `rpass2`: compilation failed!
[01:09:40] status: exit code: 101
[01:09:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/krate-inlined.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/krate-inlined/krate-inlined.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/krate-inlined/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/krate-inlined/auxiliary"
[01:09:40] ------------------------------------------
[01:09:40] 
[01:09:40] ------------------------------------------
[01:09:40] stderr:
[01:09:40] stderr:
[01:09:40] ------------------------------------------
[01:09:40] thread 'rustc' panicked at 'assertion failed: pos < self.opaque.data().len()', src/librustc/ty/query/on_disk_cache.rs:571:9
[01:09:40] 
[01:09:40] error: internal compiler error: unexpected panic
[01:09:40] 
[01:09:40] note: the compiler unexpectedly panicked. this is a bug.
[01:09:40] note: the compiler unexpectedly panicked. this is a bug.
[01:09:40] 
[01:09:40] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:09:40] 
[01:09:40] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:09:40] 
[01:09:40] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:09:40] 
[01:09:40] ------------------------------------------
[01:09:40] 
[01:09:40] thread '[incremental] incremental/krate-inlined.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:09:40] thread '[incremental] incremental/krate-inlined.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:09:40] 
[01:09:40] ---- [incremental] incremental/remove-private-item-cross-crate/main.rs stdout ----
[01:09:40] 
[01:09:40] error in revision `rpass2`: compilation failed!
[01:09:40] status: exit code: 101
[01:09:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/remove-private-item-cross-crate/main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remove-private-item-cross-crate/main/main.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remove-private-item-cross-crate/main/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zquery-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remove-private-item-cross-crate/main/auxiliary"
[01:09:40] ------------------------------------------
[01:09:40] 
[01:09:40] ------------------------------------------
[01:09:40] stderr:
[01:09:40] stderr:
[01:09:40] ------------------------------------------
[01:09:40] thread 'rustc' panicked at 'assertion failed: pos < self.opaque.data().len()', src/librustc/ty/query/on_disk_cache.rs:571:9
[01:09:40] 
[01:09:40] error: internal compiler error: unexpected panic
[01:09:40] 
[01:09:40] note: the compiler unexpectedly panicked. this is a bug.
[01:09:40] note: the compiler unexpectedly panicked. this is a bug.
[01:09:40] 
[01:09:40] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:09:40] 
[01:09:40] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:09:40] 
[01:09:40] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:09:40] 
[01:09:40] ------------------------------------------
[01:09:40] 
[01:09:40] thread '[incremental] incremental/remove-private-item-cross-crate/main.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:09:40] thread '[incremental] incremental/remove-private-item-cross-crate/main.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:09:40] 
[01:09:40] ---- [incremental] incremental/lto.rs stdout ----
[01:09:40] 
[01:09:40] error in revision `rpass2`: compilation failed!
[01:09:40] status: exit code: 101
[01:09:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/lto.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/lto/lto.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/lto/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "lto" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/lto/auxiliary"
[01:09:40] ------------------------------------------
[01:09:40] 
[01:09:40] ------------------------------------------
[01:09:40] stderr:
[01:09:40] stderr:
[01:09:40] ------------------------------------------
[01:09:40] thread 'rustc' panicked at 'assertion failed: pos < self.opaque.data().len()', src/librustc/ty/query/on_disk_cache.rs:571:9
[01:09:40] 
[01:09:40] error: internal compiler error: unexpected panic
[01:09:40] 
[01:09:40] note: the compiler unexpectedly panicked. this is a bug.
[01:09:40] note: the compiler unexpectedly panicked. this is a bug.
[01:09:40] 
[01:09:40] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:09:40] 
[01:09:40] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:09:40] 
[01:09:40] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C rpath -C lto
[01:09:40] 
[01:09:40] ------------------------------------------
[01:09:40] 
[01:09:40] thread '[incremental] incremental/lto.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:09:40] thread '[incremental] incremental/lto.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:09:40] 
[01:09:40] ---- [incremental] incremental/static_refering_to_other_static3/issue.rs stdout ----
[01:09:40] 
[01:09:40] error in revision `rpass2`: compilation failed!
[01:09:40] status: exit code: 101
[01:09:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/static_refering_to_other_static3/issue.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/static_refering_to_other_static3/issue/issue.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/static_refering_to_other_static3/issue/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/static_refering_to_other_static3/issue/auxiliary"
[01:09:40] ------------------------------------------
[01:09:40] 
[01:09:40] ------------------------------------------
[01:09:40] stderr:
[01:09:40] stderr:
[01:09:40] ------------------------------------------
[01:09:40] thread 'rustc' panicked at 'assertion failed: pos < self.opaque.data().len()', src/librustc/ty/query/on_disk_cache.rs:571:9
[01:09:40] 
[01:09:40] error: internal compiler error: unexpected panic
[01:09:40] 
[01:09:40] note: the compiler unexpectedly panicked. this is a bug.
[01:09:40] note: the compiler unexpectedly panicked. this is a bug.
[01:09:40] 
[01:09:40] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:09:40] 
[01:09:40] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:09:40] 
[01:09:40] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath
[01:09:40] 
[01:09:40] ------------------------------------------
[01:09:40] 
[01:09:40] thread '[incremental] incremental/static_refering_to_other_static3/issue.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:09:40] thread '[incremental] incremental/static_refering_to_other_static3/issue.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:09:40] 
[01:09:40] ---- [incremental] incremental/static_stable_hash/issue-49301.rs stdout ----
[01:09:40] 
[01:09:40] error in revision `rpass2`: compilation failed!
[01:09:40] status: exit code: 101
[01:09:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/static_stable_hash/issue-49301.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/static_stable_hash/issue-49301/issue-49301.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/static_stable_hash/issue-49301/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/static_stable_hash/issue-49301/auxiliary"
[01:09:40] ------------------------------------------
[01:09:40] 
[01:09:40] ------------------------------------------
[01:09:40] stderr:
[01:09:40] stderr:
[01:09:40] ------------------------------------------
[01:09:40] thread 'rustc' panicked at 'assertion failed: pos < self.opaque.data().len()', src/librustc/ty/query/on_disk_cache.rs:571:9
[01:09:40] 
[01:09:40] error: internal compiler error: unexpected panic
[01:09:40] 
[01:09:40] note: the compiler unexpectedly panicked. this is a bug.
[01:09:40] note: the compiler unexpectedly panicked. this is a bug.
[01:09:40] 
[01:09:40] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:09:40] 
[01:09:40] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:09:40] 
[01:09:40] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath
[01:09:40] 
[01:09:40] ------------------------------------------
[01:09:40] 
[01:09:40] thread '[incremental] incremental/static_stable_hash/issue-49301.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
---
[01:09:40] test result: FAILED. 90 passed; 8 failed; 0 ignored; 0 measured; 0 filtered out
[01:09:40] 
[01:09:40] 
[01:09:40] 
[01:09:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:09:40] 
[01:09:40] 
[01:09:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:09:40] Build completed unsuccessfully in 0:11:08
[01:09:40] Build completed unsuccessfully in 0:11:08
[01:09:40] Makefile:48: recipe for target 'check' failed
[01:09:40] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0729a7a0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Feb 22 18:58:03 UTC 2019
---
travis_time:end:1190ec5b:start=1550861884867840205,finish=1550861884872031375,duration=4191170
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0115c4ea
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:034368f2
travis_time:start:034368f2
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:042ab595
$ dmesg | grep -i kill
