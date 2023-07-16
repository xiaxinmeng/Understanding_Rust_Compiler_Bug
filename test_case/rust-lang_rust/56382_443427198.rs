plain
travis_time:end:0b0575b0:start=1543668176645396531,finish=1543668234314888455,duration=57669491924
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:57:32] 
[00:57:32] running 119 tests
[00:57:35] i..ii...iii..iiii.....i...i.........i..iii.............i.....i.....ii...i..i.ii..............i...ii. 100/119
[00:57:36] .ii.i.....iiii.....
[00:57:36] 
[00:57:36]  finished in 3.438
[00:57:36] travis_fold:end:test_codegen

---
[00:57:46] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:57:46] 
[00:57:46] ---- [incremental] incremental/callee_caller_cross_crate/b.rs stdout ----
[00:57:46] 
[00:57:46] error in revision `rpass2`: auxiliary build of "/checkout/src/test/incremental/callee_caller_cross_crate/auxiliary/a.rs" failed to compile: 
[00:57:46] status: exit code: 101
[00:57:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/callee_caller_cross_crate/auxiliary/a.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/callee_caller_cross_crate/b/b.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/callee_caller_cross_crate/b/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/callee_caller_cross_crate/b/auxiliary"
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] stderr:
[00:57:46] stderr:
[00:57:46] ------------------------------------------
[00:57:46] thread 'main' panicked at 'no entry found for key', src/li---------------
[00:57:46] thread '[incremental] incremental/change_private_fn_cc/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] 
[00:57:46] ---- [incremental] incremental/change_crate_order/main.rs stdout ----
[00:57:46] 
[00:57:46] 
[00:57:46] error in revision `rpass2`: auxiliary build of "/checkout/src/test/incremental/change_crate_order/auxiliary/a.rs" failed to compile: 
[00:57:46] status: exit code: 101
[00:57:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_crate_order/auxiliary/a.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_order/main/main.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_order/main/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_order/main/auxiliary"
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] stderr:
[00:57:46] stderr:
[00:57:46] ------------------------------------------
[00:57:46] thread 'main' panicked at 'no entry found for key', src/libcore/option.rs:1008:5
[00:57:46] 
[00:57:46] error: internal compiler error: unexpected panic
[00:57:46] 
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] 
[00:57:46] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:57:46] 
[00:57:46] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:57:46] 
[00:57:46] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath --crate-type dylib
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] thread '[incremental] incremental/change_crate_order/main.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] thread '[incremental] incremental/change_crate_order/main.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] 
[00:57:46] ---- [incremental] incremental/change_private_impl_method/struct_point.rs stdout ----
[00:57:46] 
[00:57:46] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:57:46] status: exit code: 101
[00:57:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_impl_method/struct_point.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point/a" "-Crpath" "-O" "-Zunstable-hange_private_impl_method_cc/auxiliary/point.rs" failed to compile: 
[00:57:46] status: exit code: 101
[00:57:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_impl_method_cc/auxiliary/point.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point/auxiliary"
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] stderr:
[00:57:46] stderr:
[00:57:46] ------------------------------------------
[00:57:46] thread 'main' panicked at 'no entry found for key', src/libcore/option.rs:1008:5
[00:57:46] 
[00:57:46] error: internal compiler error: unexpected panic
[00:57:46] 
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] 
[00:57:46] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:57:46] 
[00:57:46] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:57:46] 
[00:57:46] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath --crate-type dylib
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] thread '[incremental] incremental/change_private_impl_method_cc/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] thread '[incremental] incremental/change_private_impl_method_cc/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] 
[00:57:46] ---- [incremental] incremental/change_pub_inherent_method_body/struct_point.rs stdout ----
[00:57:46] 
[00:57:46] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:57:46] status: exit code: 101
[00:57:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_pub_inherent_method_body/struct_point.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point/auxiliary"
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] stderr:
[00:57:46] stderr:
[00:57:46] ------------------------------------------
[00:57:46] thread 'main' panicked at 'no entry found for key', src/libcore/option.rs:1008:5
[00:57:46] 
[00:57:46] error: internal compiler error: unexpected panic
[00:57:46] 
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] 
[00:57:46] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:57:46] 
[00:57:46] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:57:46] 
[00:57:46] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] thread '[incremental] incremental/change_pub_inherent_method_body/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] thread '[incremental] incremental/change_pub_inherent_method_body/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] 
[00:57:46] ---- [incremental] incremental/change_pub_inherent_method_sig/struct_point.rs stdout ----
[00:57:46] 
[00:57:46] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:57:46] status: exit code: 101
[00:57:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_pub_inherent_method_sig/struct_point.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cffile_name":"/checkout/src/test/incremental/change_symbol_export_status.rs","byte_start":566,"byte_end":587,"line_start":15,"line_end":15,"column_start":10,"column_end":31,"is_primary":true,"text":[{"text":"#![allow(private_no_mangle_fns)]","highlight_start":10,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(renamed_and_removed_lints)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: lint `private_no_mangle_fns` has been removed: `no longer an warning, #[no_mangle] functions always exported`\n  --> /checkout/src/test/incremental/change_symbol_export_status.rs:15:10\n   |\nLL | #![allow(private_no_mangle_fns)]\n   |          ^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(renamed_and_removed_lints)] on by default\n\n"}
[00:57:46] thread 'main' panicked at 'no entry found for key', src/libcore/option.rs:1008:5
[00:57:46] 
[00:57:46] error: internal compiler error: unexpected panic
[00:57:46] 
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] 
[00:57:46] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:57:46] 
[00:57:46] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:57:46] 
[00:57:46] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[00:57:46] 
[00:57:46] 
[00:57:46] ---------cremental/extern_static/issue-49153/issue-49153.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/extern_static/issue-49153/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/extern_static/issue-49153/auxiliary"
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] stderr:
[00:57:46] stderr:
[00:57:46] ------------------------------------------
[00:57:46] thread 'main' panicked at 'no entry found for key', src/libcore/option.rs:1008:5
[00:57:46] 
[00:57:46] error: internal compiler error: unexpected panic
[00:57:46] 
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] 
[00:57:46] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:57:46] 
[00:57:46] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:57:46] 
[00:57:46] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] thread '[incremental] incremental/extern_static/issue-49153.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] thread '[incremental] incremental/extern_static/issue-49153.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] 
[00:57:46] ---- [incremental] incremental/commandline-args.rs stdout ----
[00:57:46] 
[00:57:46] error in revision `rpass3`: compilation failed!
[00:57:46] status: exit code: 101
[00:57:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/commandline-args.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/commandline-args/commandline-args.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/commandline-args/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-C" "debuginfo=2" "--verbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/commandline-args/auxiliary"
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] stderr:
[00:57:46] stderr:
[00:57:46] ------------------------------------------
[00:57:46] thread 'main' panicked at 'no entry found for key', src/libcore/option.rs:1008:5
[00:57:46] 
[00:57:46] error: internal compiler error: unexpected panic
[00:57:46] 
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] 
[00:57:46] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:57:46] 
[00:57:46] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:57:46] 
[00:57:46] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=2
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] thread '[incremental] incremental/commandline-args.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] thread '[incremental] incremental/commandline-args.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] 
[00:57:46] ---- [incremental] incremental/hashes/call_expressions.rs stdout ----
[00:57:46] 
[00:57:46] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:57:46] status: exit code: 101
[00:57:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/call_expressions.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions/call_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions/auxiliary"
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] stderr:
[00:57:46] stderr:
[00:57:46] ------------------------------------------
[00:57:46] thread 'main' panicked at 'no entry found for key', src/libcore/option.rs:1008:5
[00:57:46] 
[00:57:46] error: internal compiler error: unexpected panic
[00:57:46] 
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] 
[00:57:46] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:57:46] 
[00:57:46] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:57:46] 
[00:57:46] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] thread '[incremental] incremental/hashes/call_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] thread '[incremental] incremental/hashes/call_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] 
[00:57:46] ---- [incremental] incremental/hashes/closure_expressions.rs stdout ----
[00:57:46] 
[00:57:46] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:57:46] status: exit code: 101
[00:57:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/closure_expressions.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/closure_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/auxiliary"
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] stderr:
[00:57:46] stderr:
[00:57:46] ------------------------------------------
[00:57:46] thread 'main' panicked at 'no entry found for key', src/libcore/option.rs:1008:5
[00:57:46] 
[00:57:46] error: internal compiler error: unexpected panic
[00:57:46] 
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] 
[00:57:46] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:57:46] 
[00:57:46] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:57:46] 
[00:57:46] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] 
[00:57:46] thread '[incremental] incremental/hashes/closors/enum_constructors.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors/auxiliary"
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] stderr:
[00:57:46] stderr:
[00:57:46] ------------------------------------------
[00:57:46] thread 'main' panicked at 'no entry found for key', src/libcore/option.rs:1008:5
[00:57:46] 
[00:57:46] error: internal compiler error: unexpected panic
[00:57:46] 
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] 
[00:57:46] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:57:46] 
[00:57:46] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:57:46] 
[00:57:46] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] thread '[incremental] incremental/hashes/enum_constructors.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] thread '[incremental] incremental/hashes/enum_constructors.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] 
[00:57:46] ---- [incremental] incremental/hashes/exported_vs_not.rs stdout ----
[00:57:46] 
[00:57:46] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:57:46] status: exit code: 101
[00:57:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/exported_vs_not.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not/exported_vs_not.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not/auxiliary"
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] stderr:
[00:57:46] stderr:
[00:57:46] ------------------------------------------
[00:57:46] thread 'main' panicked at 'no entry found for key', src/libcore/option.rs:1008:5
[00:57:46] 
[00:57:46] error: internal compiler error: unexpected panic
[00:57:46] 
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
---
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] stderr:
[00:57:46] ------------------------------------------
[00:57:46] thread 'main' panicked at 'no entry found for key', src/libcore/option.rs:1008:5
[00:57:46] 
[00:57:46] error: internal compiler error: unexpected panic
[00:57:46] 
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] 
[00:57:46] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:57:46] 
[00:57:46] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:57:46] 
[00:57:46] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] thread '[incremental] incremental/hashes/enum_defs.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] thread '[incremental] incremental/hashes/enum_defs.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] 
[00:57:46] ---- [incremental] incremental/hashes/if_expressions.rs stdout ----
[00:57:46] 
[00:57:46] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:57:46] status: exit code: 101
[00:57:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/if_expressions.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions/if_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions/auxiliary"
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] stderr:
[00:57:46] stderr:
[00:57:46] ------------------------------------------
[00:57:46] thread 'main' panicked at 'no entry found for key', src/libcore/option.rs:1008:5
[00:57:46] 
[00:57:46] error: internal compiler error: unexpected panic
[00:57:46] 
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] 
[00:57:46] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:57:46] 
[00:57:46] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:57:46] 
[00:57:46] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] thread '[incremental] incremental/hashes/if_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] thread '[incremental] incremental/hashes/if_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] 
[00:57:46] ---- [incremental] incremental/hashes/indexing_expressions.rs stdout ----
[00:57:46] 
[00:57:46] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:57:46] status: exit code: 101
[00:57:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/indexing_expressions.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/indexing_expressions/indexing_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/indexing_expressions/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/indexing_expressions/auxiliary"
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] stderr:
[00:57:46] stderr:
[00:57:46] ------------------------------------------
[00:57:46] thread 'main' panicked at 'no entry found for key', src/libcore/option.rs:1008:5
[00:57:46] 
[00:57:46] 
[00:57:46] error: internal compiler error: unexpeout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/for_loops.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/for_loops.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/auxiliary"
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] stderr:
[00:57:46] stderr:
[00:57:46] ------------------------------------------
[00:57:46] thread 'main' panicked at 'no entry found for key', src/libcore/option.rs:1008:5
[00:57:46] 
[00:57:46] error: internal compiler error: unexpected panic
[00:57:46] 
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] 
[00:57:46] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:57:46] 
[00:57:46] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:57:46] 
[00:57:46] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z intus: exit code: 101
[00:57:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/let_expressions.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions/let_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions/auxiliary"
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] stderr:
[00:57:46] stderr:
[00:57:46] ------------------------------------------
[00:57:46] thread 'main' panicked at 'no entry found for key', src/libcore/option.rs:1008:5
[00:57:46] 
[00:57:46] error: internal compiler error: unexpected panic
[00:57:46] 
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] 
[00:57:46] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:57:46] 
[00:57:46] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:57:46] 
[00:57:46] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] thread '[incremental] incremental/hashes/let_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] thread '[incremental] incremental/hashes/let_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] 
[00:57:46] ---- [incremental] incremental/hashes/loop_expressions.rs stdout ----
[00:57:46] 
[00:57:46] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:57:46] status: exit code: 101
[00:57:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/loop_expressions.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/loop_expressions/loop_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/loop_expressions/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/loop_expressions/auxiliary"
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] stderr:
[00:57:46] stderr:
[00:57:46] ------------------------------------------
[00:57:46] thread 'main' panicked at 'no entry found for key', src/libcore/option.rs:1008:5
[00:57:46] 
[00:57:46] error: internal compiler error: unexpected panic
[00:57:46] 
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] 
[00:57:46] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:57:46] 
[00:57:46] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:57:46] 
[00:57:46] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] thread '[incremental] incremental/hashes/loop_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] thread '[incremental] incremental/hashes/loop_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] 
[00:57:46] ---- [incremental] incremental/hashes/match_expressions.rs stdout ----
[00:57:46] 
[00:57:46] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:57:46] status: exit code: 101
[00:57:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/match_expressions.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/match_expressions/match_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/match_expressions/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/match_expressions/auxiliary"
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] stderr:
[00:57:46] stderr:
[00:57:46] ------------------------------------------
[00:57:46] thread 'main' panicked at 'no entry found for key', src/libcore/option.rs:1008:5
[00:57:46] 
[00:57:46] error: internal compiler error: unexpected panic
[00:57:46] 
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] 
[00:57:46] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:57:46] 
[00:57:46] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:57:46] 
[00:57:46] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] thread '[incremental] incremental/hashes/match_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] thread '[incremental] incremental/hashes/match_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] 
[00:57:46] ---- [incremental] incremental/hashesy-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/panic_exprs/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-C" "debug-assertions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/panic_exprs/auxiliary"
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] stderr:
[00:57:46] stderr:
[00:57:46] ------------------------------------------
[00:57:46] thread 'main' panicked at 'no entry found for key', src/libcore/option.rs:1008:5
[00:57:46] 
[00:57:46] error: internal compiler error: unexpected panic
[00:57:46] 
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] 
[00:57:46] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:57:46] 
[00:57:46] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:57:46] 
[00:57:46] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debug-assertions
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] thread '[incremental] incremental/hashes/panic_exprs.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] thread '[incremental] incremental/hashes/panic_exprs.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] 
[00:57:46] ---- [incremental] incremental/hashes/trait_impls.rs stdout ----
[00:57:46] 
[00:57:46] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:57:46] status: exit code: 101
[00:57:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/trait_impls.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_impls/trait_impls.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_impls/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_impls/auxiliary"
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] stderr:
[00:57:46] stderr:
[00:57:46] ------------------------------------------
[00:57:46] thread 'main' panicked at 'no entry found for key', src/libcore/option.rs:1008:5
[00:57:46] 
[00:57:46] error: internal compiler error: unexpected panic
[00:57:46] 
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] 
[00:57:46] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBU00:57:46] ------------------------------------------
[00:57:46] stderr:
[00:57:46] ------------------------------------------
[00:57:46] thread 'main' panicked at 'no entry found for key', src/libcore/option.rs:1008:5
[00:57:46] 
[00:57:46] error: internal compiler error: unexpected panic
[00:57:46] 
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] 
[00:57:46] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:57:46] 
[00:57:46] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:57:46] 
[00:57:46] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] thread '[incremental] incremental/hashes/trait_defs.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] thread '[incremental] incremental/hashes/trait_defs.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] 
[00:57:46] ---- [incremental] incremental/hashes/struct_defs.rs stdout ----
[00:57:46] 
[00:57:46] error in revision `cfail3`: test compilation failed although it shouldn't!
[00:57:46] status: exit code: 101
[00:57:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/struct_defs.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/struct_defs/struct_defs.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/struct_defs/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/struct_defs/auxiliary"
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] stderr:
[00:57:46] stderr:
[00:57:46] ------------------------------------------
[00:57:46] thread 'main' panicked at 'no entry found for key', src/libcore/option.rs:1008:5
[00:57:46] thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
[00:57:46]   left: `LLVMing`,
[00:57:46]   left: `LLVMing`,
[00:57:46]  right: `Codegenning`', src/librustc_codegen_ssa/back/write.rs:1358:21
[00:57:46] error: internal compiler error: unexpected panic
[00:57:46] 
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] 
[00:57:46] 
[00:57:46] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:57:46] 
[00:57:46] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:57:46] 
[00:57:46] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] thread '[incremental] incremental/hashes/struct_defs.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] thread '[incremental] incremental/hashes/struct_defs.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] 
[00:57:46] ---- [incremental] incremental/hashes/while_let_loops.rs stdout ----
[00:57:46] 
[00:57:46] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:57:46] status: exit code: 101
[00:57:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/while_let_loops.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_let_loops/while_let_loops.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_let_loops/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_let_loops/auxiliary"
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] stderr:
[00:57:46] stderr:
[00:57:46] ------------------------------------------
[00:57:46] thread 'main' panicked at 'no entry found for key', src/libcore/option.rs:1008:5
[00:57:46] 
[00:57:46] 
[00:57:4/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/unary_and_binary_exprs.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/unary_and_binary_exprs/unary_and_binary_exprs.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/unary_and_binary_exprs/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/unary_and_binary_exprs/auxiliary"
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] stderr:
[00:57:46] stderr:
[00:57:46] ------------------------------------------
[00:57:46] thread 'main' panicked at 'no entry found for key', src/libcore/option.rs:1008:5
[00:57:46] 
[00:57:46] error: internal compiler error: unexpected panic
[00:57:46] 
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] 
[00:57:46] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:57:46] 
[00:57:46] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:57:46] 
[00:57:46] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries ux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/ich_method_call_trait_scope/ich_method_call_trait_scope.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/ich_method_call_trait_scope/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/ich_method_call_trait_scope/auxiliary"
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] stderr:
[00:57:46] stderr:
[00:57:46] ------------------------------------------
[00:57:46] thread 'main' panicked at 'no entry found for key', src/libcore/option.rs:1008:5
[00:57:46] 
[00:57:46] error: internal compiler error: unexpected panic
[00:57:46] 
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] 
[00:57:46] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:57:46] 
[00:57:46] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:57:46] 
[00:57:46] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] thread '[incremental] incremental/ich_method_call_trait_scope.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] thread '[incremental] incremental/ich_method_call_trait_scope.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] 
[00:57:46] ---- [incremental] incremental/ich_resolve_results.rs stdout ----
[00:57:46] 
[00:57:46] error in revision `rpass2`: compilation failed!
[00:57:46] status: exit code: 101
[00:57:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/ich_resolve_results.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/ich_resolve_results/ich_resolve_results.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/ich_resolve_results/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/ich_resolve_results/auxiliary"
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] stderr:
[00:57:46] stderr:
[00:57:46] ------------------------------------------
[00:57:46] thread 'main' panicked at 'no entry found for key', src/libcore/option.rs:1008:5
[00:57:46] 
[00:57:46] error: internal compiler error: unexpected panic
[00:57:46] 
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] 
[00:57:46] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:57:46] 
[00:57:46] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:57:46] 
[00:57:46] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] thread '[incremental] incremental/ich_resolve_results.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] thread '[incremental] incremental/ich_resolve_results.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] 
[00:57:46] ---- [incremental] incremental/issue-35593.rs stdout ----
[00:57:46] 
[00:57:46] error in revision `rpass2`: compilation failed!
[00:57:46] status: exit code: 101
[00:57:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-35593.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-35593/issue-35593.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-35593/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-35593/auxiliary"
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] stderr:
[00:57:46] stderr:
[00:57:46] ------------------------------------------
[00:57:46] thread 'main' panicked at 'no entry found for key', src/libcore/option.rs:1008:5
[00:57:46] 
[00:57:46] error: internal compiler error: unexpected panic
[00:57:46] 
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] 
[00:57:46] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:57:46] 
[00:57:46] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:57:46] 
[00:57:46] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] thread '[incremental] incremental/issue-35593.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] thread '[incremental] incremental/issue-35593.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] 
[00:57:46] ---- [incremental] incremental/inlined_hir_34991/main.rs stdout ----
[00:57:46] 
[00:57:46] error in revision `rpass2`: compilation failed!
[00:57:46] status: exit code: 101
[00:57:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/inlined_hir_34991/main.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/inlined_hir_34991/main/main.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "o entry found for key', src/libcore/option.rs:1008:5
[00:57:46] 
[00:57:46] error: internal compiler error: unexpected panic
[00:57:46] 
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] 
[00:57:46] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:57:46] 
[00:57:46] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:57:46] 
[00:57:46] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C rpath
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] thread '[incremental] incremental/incremental_proc_macro.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] thread '[incremental] incremental/incremental_proc_macro.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] 
[00:57:46] ---- [incremental] incremental/issue-39569.rs stdout ----
[00:57:46] 
[00:57:46] error in revision `rpass2`: compilation failed!
[00:57:46] status: exit code: 101
[00:57:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-39569.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-39569/issue-39569.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-39569/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-39569/auxiliary"
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] stderr:
[00:57:46] stderr:
[00:57:46] ------------------------------------------
[00:57:46] {"message":"unused variable: `y`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/incremental/issue-39569.rs","byte_start":1056,"byte_end":1057,"line_start":36,"line_end":36,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    let y: Arc<FooX> = x;","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_variables)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `_y` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/incremental/issue-39569.rs","byte_start":1056,"byte_end":1057,"line_start":36,"line_end":36,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    let y: Arc<FooX> = x;","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"_y","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `y`\n  --> /checkout/src/test/incremental/issue-39569.rs:36:9\n   |\nLL |     let y: Arc<FooX> = x;\n   |         ^ help: consider using `_y` instead0:57:46] ------------------------------------------
[00:57:46] thread '[incremental] incremental/issue-39569.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] 
[00:57:46] ---- [incremental] incremental/issue-39828/issue-39828.rs stdout ----
[00:57:46] 
[00:57:46] 
[00:57:46] error in revision `rpass2`: auxiliary build of "/checkout/src/test/incremental/issue-39828/auxiliary/generic.rs" failed to compile: 
[00:57:46] status: exit code: 101
[00:57:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-39828/auxiliary/generic.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-39828/issue-39828/issue-39828.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-39828/issue-39828/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-39828/issue-39828/auxiliary"
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] stderr:
[00:57:46] stderr:
[00:57:46] ------------------------------------------
[00:57:46] thread 'main' panicked at 'no entry found for key', src/libcore/option.rs:1008:5
[00:57:46] 
[00:57:46] error: internal compiler error: unexpected panic
[00:57:46] 
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] 
[00:57:46] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:57:46] 
[00:57:46] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:57:46] 
[00:57:46] make: *** [check] Error 1
[00:57:46] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath --crate-type dylib
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] thread '[incremental] incremental/issue-39828/issue-39828.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] thread '[incremental] incremental/issue-39828/issue-39828.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] 
[00:57:46] ---- [incremental] incremental/issue-42602.rs stdout ----
[00:57:46] 
[00:57:46] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:57:46] status: exit code: 101
[00:57:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-42602.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-42602/issue-42602.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-42602/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zquery-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-42602/auxiliary"
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] stderr:
[00:57:46] stderr:
[00:57:46] ------------------------------------------
[00:57:46] thread 'main' panicked at 'no entry found for key', src/libcore/option.rs:1008:5
[00:57:46] 
[00:57:46] error: internal compiler error: unexpected panic
[00:57:46] 
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] 
[00:57:46] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:57:46] 
[00:57:46] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:57:46] 
[00:57:46] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] thread '[incremental] incremental/issue-42602.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] thread '[incremental] incremental/issue-42602.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] 
[00:57:46] ---- [incremental] incremental/issue-49595/issue_49595.rs stdout ----
[00:57:46] 
[00:57:46] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:57:46] command: "/checkout/obj/build/x86_64-unknown-linuxpected panic
[00:57:46] 
[00:57:46] note: the compiler unexpectedly panicked. this is a bug.
[00:57:46] 
[00:57:46] 
[00:57:46] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:57:46] 
[00:57:46] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:57:46] 
[00:57:46] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] thread '[incremental] incremental/krate-inherent.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] thread '[incremental] incremental/krate-inherent.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:57:46] 
[00:57:46] ---- [incremental] incremental/krate-inlined.rs stdout ----
[00:57:46] 
[00:57:46] error in revision `rpass2`: compilation failed!
[00:57:46] status: exit code: 101
[00:57:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/krate-inlined.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/krate-inlined/krate-inlined.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/krate-inlined/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/krate-ntest.rs:3282:9
[00:57:46] ---- [incremental] incremental/remapped_paths_cc/main.rs stdout ----
[00:57:46] 
