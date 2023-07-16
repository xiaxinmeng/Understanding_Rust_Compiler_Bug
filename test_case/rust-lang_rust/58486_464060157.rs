plain
travis_time:end:00095cf4:start=1550235010451125583,finish=1550235012056092877,duration=1604967294
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
[01:10:25] 
[01:10:25] running 98 tests
[01:10:38] .F..........F................F.....................F..F.F..F..............F...F...F.F.............
[01:10:38] 
[01:10:38] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:10:38] ---- [incremental] incremental/change_add_field/struct_point.rs stdout ----
[01:10:38] 
[01:10:38] 
[01:10:38] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:10:38] status: exit code: 101
[01:10:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_add_field/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point/auxiliary"
[01:10:38] ------------------------------------------
[01:10:38] 
[01:10:38] ------------------------------------------
[01:10:38] stderr:
[01:10:38] stderr:
[01:10:38] ------------------------------------------
[01:10:38] thread 'rustc' panicked at 'Found unstable fingerprints for NormalizeTyAfterErasingRegions(ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All, def_id: None }, value: [usize; _] })', src/librustc/ty/query/plumbing.rs:528:9
[01:10:38] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:10:38] thread '<unnamed>' panicked at 'Metadata module not compiled?', src/libcore/option.rs:1038:5
[01:10:38] error: internal compiler error: unexpected panic
[01:10:38] 
[01:10:38] note: the compiler unexpectedly panicked. this is a bug.
[01:10:38] 
[01:10:38] 
[01:10:38] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:10:38] 
[01:10:38] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:10:38] 
[01:10:38] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:10:38] 
[01:10:38] ------------------------------------------
[01:10:38] 
[01:10:38] thread '[incremental] incremental/change_add_field/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:10:38] thread '[incremental] incremental/change_add_field/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:10:38] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:10:38] 
[01:10:38] ---- [incremental] incremental/change_pub_inherent_method_sig/struct_point.rs stdout ----
[01:10:38] 
[01:10:38] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:10:38] status: exit code: 101
[01:10:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_pub_inherent_method_sig/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point/auxiliary"
[01:10:38] ------------------------------------------
[01:10:38] 
[01:10:38] ------------------------------------------
[01:10:38] stderr:
[01:10:38] stderr:
[01:10:38] ------------------------------------------
[01:10:38] thread 'rustc' panicked at 'Found unstable fingerprints for NormalizeTyAfterErasingRegions(ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All, def_id: None }, value: [usize; _] })', src/librustc/ty/query/plumbing.rs:528:9
[01:10:38] 
[01:10:38] error: internal compiler error: unexpected panic
[01:10:38] 
[01:10:38] note: the compiler unexpectedly panicked. this is a bug.
[01:10:38] note: the compiler unexpectedly panicked. this is a bug.
[01:10:38] 
[01:10:38] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:10:38] 
[01:10:38] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:10:38] 
[01:10:38] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:10:38] 
[01:10:38] ------------------------------------------
[01:10:38] 
[01:10:38] thread '[incremental] incremental/change_pub_inherent_method_sig/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:10:38] thread '[incremental] incremental/change_pub_inherent_method_sig/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:10:38] 
[01:10:38] ---- [incremental] incremental/hashes/for_loops.rs stdout ----
[01:10:38] 
[01:10:38] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:10:38] status: exit code: 101
[01:10:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/for_loops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/for_loops.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/auxiliary"
[01:10:38] ------------------------------------------
[01:10:38] 
[01:10:38] ------------------------------------------
[01:10:38] stderr:
[01:10:38] stderr:
[01:10:38] ------------------------------------------
[01:10:38] thread 'rustc' panicked at 'Found unstable fingerprints for MirOptimized(core[255e]::iter[0]::range[0]::{{impl}}[0]::next[0])', src/librustc/ty/query/plumbing.rs:528:9
[01:10:38] 
[01:10:38] error: internal compiler error: unexpected panic
[01:10:38] 
[01:10:38] note: the compiler unexpectedly panicked. this is a bug.
[01:10:38] note: the compiler unexpectedly panicked. this is a bug.
[01:10:38] 
[01:10:38] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:10:38] 
[01:10:38] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:10:38] 
[01:10:38] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[01:10:38] 
[01:10:38] ------------------------------------------
[01:10:38] 
[01:10:38] thread '[incremental] incremental/hashes/for_loops.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:10:38] thread '[incremental] incremental/hashes/for_loops.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:10:38] 
[01:10:38] ---- [incremental] incremental/incremental_proc_macro.rs stdout ----
[01:10:38] 
[01:10:38] error in revision `cfail2`: auxiliary build of "/checkout/src/test/incremental/auxiliary/incremental_proc_macro_aux.rs" failed to compile: 
[01:10:38] status: exit code: 101
[01:10:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/auxiliary/incremental_proc_macro_aux.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/incremental_proc_macro/incremental_proc_macro.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/incremental_proc_macro/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/incremental_proc_macro/auxiliary"
[01:10:38] ------------------------------------------
[01:10:38] 
[01:10:38] ------------------------------------------
[01:10:38] stderr:
[01:10:38] stderr:
[01:10:38] ------------------------------------------
[01:10:38] {"message":"unused variable: `input`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/incremental/auxiliary/incremental_proc_macro_aux.rs","byte_start":262,"byte_end":267,"line_start":15,"line_end":15,"column_start":15,"column_end":20,"is_primary":true,"text":[{"text":"pub fn derive(input: TokenStream) -> TokenStream {","highlight_start":15,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_variables)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider prefixing with an underscore","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/incremental/auxiliary/incremental_proc_macro_aux.rs","byte_start":262,"byte_end":267,"line_start":15,"line_end":15,"column_start":15,"column_end":20,"is_primary":true,"text":[{"text":"pub fn derive(input: TokenStream) -> TokenStream {","highlight_start":15,"highlight_end":20}],"label":null,"suggested_replacement":"_input","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `input`\n  --> /checkout/src/test/incremental/auxiliary/incremental_proc_macro_aux.rs:15:15\n   |\nLL | pub fn derive(input: TokenStream) -> TokenStream {\n   |               ^^^^^ help: consider prefixing with an underscore: `_input`\n   |\n   = note: #[warn(unused_variables)] on by default\n\n"}
[01:10:38] thread 'rustc' panicked at 'Found unstable fingerprints for NormalizeTyAfterErasingRegions(2e42b4c9fadc4b6c-d7f0ed43453b5699)', src/librustc/ty/query/plumbing.rs:528:9
[01:10:38] 
[01:10:38] error: internal compiler error: unexpected panic
[01:10:38] 
[01:10:38] note: the compiler unexpectedly panicked. this is a bug.
[01:10:38] note: the compiler unexpectedly panicked. this is a bug.
[01:10:38] 
[01:10:38] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:10:38] 
[01:10:38] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:10:38] 
[01:10:38] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C rpath
[01:10:38] 
[01:10:38] ------------------------------------------
[01:10:38] 
[01:10:38] thread '[incremental] incremental/incremental_proc_macro.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:10:38] thread '[incremental] incremental/incremental_proc_macro.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:10:38] 
[01:10:38] ---- [incremental] incremental/issue-39569.rs stdout ----
[01:10:38] 
[01:10:38] error in revision `rpass2`: compilation failed!
[01:10:38] status: exit code: 101
[01:10:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-39569.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-39569/issue-39569.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-39569/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-39569/auxiliary"
[01:10:38] ------------------------------------------
[01:10:38] 
[01:10:38] ------------------------------------------
[01:10:38] stderr:
[01:10:38] stderr:
[01:10:38] ------------------------------------------
[01:10:38] {"message":"unused variable: `y`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/incremental/issue-39569.rs","byte_start":589,"byte_end":590,"line_start":26,"line_end":26,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    let y: Arc<FooX> = x;","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_variables)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider prefixing with an underscore","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/incremental/issue-39569.rs","byte_start":589,"byte_end":590,"line_start":26,"line_end":26,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    let y: Arc<FooX> = x;","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"_y","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `y`\n  --> /checkout/src/test/incremental/issue-39569.rs:26:9\n   |\nLL |     let y: Arc<FooX> = x;\n   |         ^ help: consider prefixing with an underscore: `_y`\n   |\n   = note: #[warn(unused_variables)] on by default\n\n"}
[01:10:38] {"message":"field is never used: `x`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/incremental/issue-39569.rs","byte_start":493,"byte_end":501,"line_start":21,"line_end":21,"column_start":15,"column_end":23,"is_primary":true,"text":[{"text":"struct FooX { x: usize }","highlight_start":15,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: field is never used: `x`\n  --> /checkout/src/test/incremental/issue-39569.rs:21:15\n   |\nLL | struct FooX { x: usize }\n   |               ^^^^^^^^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
[01:10:38] thread 'rustc' panicked at 'Found unstable fingerprints for NormalizeTyAfterErasingRegions(ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All, def_id: None }, value: [usize; _] })', src/librustc/ty/query/plumbing.rs:528:9
[01:10:38] 
[01:10:38] error: internal compiler error: unexpected panic
[01:10:38] 
[01:10:38] note: the compiler unexpectedly panicked. this is a bug.
[01:10:38] note: the compiler unexpectedly panicked. this is a bug.
[01:10:38] 
[01:10:38] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:10:38] 
[01:10:38] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:10:38] 
[01:10:38] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:10:38] 
[01:10:38] ------------------------------------------
[01:10:38] 
[01:10:38] thread '[incremental] incremental/issue-39569.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:10:38] thread '[incremental] incremental/issue-39569.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:10:38] 
[01:10:38] ---- [incremental] incremental/issue-42602.rs stdout ----
[01:10:38] 
[01:10:38] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:10:38] status: exit code: 101
[01:10:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-42602.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-42602/issue-42602.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-42602/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zquery-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-42602/auxiliary"
[01:10:38] ------------------------------------------
[01:10:38] 
[01:10:38] ------------------------------------------
[01:10:38] stderr:
[01:10:38] stderr:
[01:10:38] ------------------------------------------
[01:10:38] thread 'rustc' panicked at 'Found unstable fingerprints for NormalizeTyAfterErasingRegions(ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All, def_id: None }, value: [usize; _] })', src/librustc/ty/query/plumbing.rs:528:9
[01:10:38] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:10:38] thread '<unnamed>' panicked at 'Metadata module not compiled?', src/libcore/option.rs:1038:5
[01:10:38] error: internal compiler error: unexpected panic
[01:10:38] 
[01:10:38] note: the compiler unexpectedly panicked. this is a bug.
[01:10:38] 
[01:10:38] 
[01:10:38] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:10:38] 
[01:10:38] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:10:38] 
[01:10:38] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:10:38] 
[01:10:38] ------------------------------------------
[01:10:38] 
[01:10:38] thread '[incremental] incremental/issue-42602.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:10:38] thread '[incremental] incremental/issue-42602.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:10:38] 
[01:10:38] ---- [incremental] incremental/issue-49595/issue_49595.rs stdout ----
[01:10:38] 
[01:10:38] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:10:38] status: exit code: 101
[01:10:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-49595/issue_49595.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-49595/issue_49595/issue_49595.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-49595/issue_49595/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-49595/issue_49595/auxiliary"
[01:10:38] ------------------------------------------
[01:10:38] 
[01:10:38] ------------------------------------------
[01:10:38] stderr:
[01:10:38] stderr:
[01:10:38] ------------------------------------------
[01:10:38] thread 'rustc' panicked at 'Found unstable fingerprints for NormalizeTyAfterErasingRegions(ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All, def_id: None }, value: [usize; _] })', src/librustc/ty/query/plumbing.rs:528:9
[01:10:38] 
[01:10:38] error: internal compiler error: unexpected panic
[01:10:38] 
[01:10:38] note: the compiler unexpectedly panicked. this is a bug.
[01:10:38] note: the compiler unexpectedly panicked. this is a bug.
[01:10:38] 
[01:10:38] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:10:38] 
[01:10:38] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:10:38] 
[01:10:38] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:10:38] 
[01:10:38] ------------------------------------------
[01:10:38] 
[01:10:38] thread '[incremental] incremental/issue-49595/issue_49595.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:10:38] thread '[incremental] incremental/issue-49595/issue_49595.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:10:38] 
[01:10:38] ---- [incremental] incremental/spans_significant_w_debuginfo.rs stdout ----
[01:10:38] 
[01:10:38] error in revision `rpass2`: compilation failed!
[01:10:38] status: exit code: 101
[01:10:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/spans_significant_w_debuginfo.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_significant_w_debuginfo/spans_significant_w_debuginfo.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_significant_w_debuginfo/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-g" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_significant_w_debuginfo/auxiliary"
[01:10:38] ------------------------------------------
[01:10:38] 
[01:10:38] ------------------------------------------
[01:10:38] stderr:
[01:10:38] stderr:
[01:10:38] ------------------------------------------
[01:10:38] thread 'rustc' panicked at 'Found unstable fingerprints for NormalizeTyAfterErasingRegions(ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All, def_id: None }, value: [usize; _] })', src/librustc/ty/query/plumbing.rs:528:9
[01:10:38] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:10:38] thread '<unnamed>' panicked at 'Metadata module not compiled?', src/libcore/option.rs:1038:5
[01:10:38] error: internal compiler error: unexpected panic
[01:10:38] 
[01:10:38] note: the compiler unexpectedly panicked. this is a bug.
[01:10:38] 
[01:10:38] 
[01:10:38] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:10:38] 
[01:10:38] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:10:38] 
[01:10:38] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:10:38] 
[01:10:38] ------------------------------------------
[01:10:38] 
[01:10:38] thread '[incremental] incremental/spans_significant_w_debuginfo.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:10:38] thread '[incremental] incremental/spans_significant_w_debuginfo.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:10:38] 
[01:10:38] ---- [incremental] incremental/spans_significant_w_panic.rs stdout ----
[01:10:38] 
[01:10:38] error in revision `rpass2`: compilation failed!
[01:10:38] status: exit code: 101
[01:10:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/spans_significant_w_panic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_significant_w_panic/spans_significant_w_panic.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_significant_w_panic/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "overflow-checks=on" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_significant_w_panic/auxiliary"
[01:10:38] ------------------------------------------
[01:10:38] 
[01:10:38] ------------------------------------------
[01:10:38] stderr:
[01:10:38] stderr:
[01:10:38] ------------------------------------------
[01:10:38] thread 'rustc' panicked at 'Found unstable fingerprints for NormalizeTyAfterErasingRegions(2e42b4c9fadc4b6c-d7f0ed43453b5699)', src/librustc/ty/query/plumbing.rs:528:9
[01:10:38] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:10:38] thread '<unnamed>' panicked at 'Metadata module not compiled?', src/libcore/option.rs:1038:5
[01:10:38] error: internal compiler error: unexpected panic
[01:10:38] 
[01:10:38] note: the compiler unexpectedly panicked. this is a bug.
[01:10:38] 
[01:10:38] 
[01:10:38] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:10:38] 
[01:10:38] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:10:38] 
[01:10:38] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath -C overflow-checks=on
[01:10:38] 
[01:10:38] ------------------------------------------
[01:10:38] 
[01:10:38] thread '[incremental] incremental/spans_significant_w_panic.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:10:38] thread '[incremental] incremental/spans_significant_w_panic.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:10:38] 
[01:10:38] ---- [incremental] incremental/lto.rs stdout ----
[01:10:38] 
[01:10:38] error in revision `rpass2`: compilation failed!
[01:10:38] status: exit code: 101
[01:10:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/lto.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/lto/lto.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/lto/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "lto" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/lto/auxiliary"
[01:10:38] ------------------------------------------
[01:10:38] 
[01:10:38] ------------------------------------------
[01:10:38] stderr:
[01:10:38] stderr:
[01:10:38] ------------------------------------------
[01:10:38] thread 'rustc' panicked at 'Found unstable fingerprints for NormalizeTyAfterErasingRegions(2e42b4c9fadc4b6c-d7f0ed43453b5699)', src/librustc/ty/query/plumbing.rs:528:9
[01:10:38] 
[01:10:38] error: internal compiler error: unexpected panic
[01:10:38] 
[01:10:38] note: the compiler unexpectedly panicked. this is a bug.
[01:10:38] note: the compiler unexpectedly panicked. this is a bug.
[01:10:38] 
[01:10:38] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:10:38] 
[01:10:38] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:10:38] 
[01:10:38] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C rpath -C lto
[01:10:38] 
[01:10:38] ------------------------------------------
[01:10:38] 
[01:10:38] thread '[incremental] incremental/lto.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:10:38] thread '[incremental] incremental/lto.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:10:38] 
[01:10:38] ---- [incremental] incremental/static_stable_hash/issue-49301.rs stdout ----
[01:10:38] 
[01:10:38] error in revision `rpass2`: compilation failed!
[01:10:38] status: exit code: 101
[01:10:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/static_stable_hash/issue-49301.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/static_stable_hash/issue-49301/issue-49301.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/static_stable_hash/issue-49301/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/static_stable_hash/issue-49301/auxiliary"
[01:10:38] ------------------------------------------
[01:10:38] 
[01:10:38] ------------------------------------------
[01:10:38] stderr:
[01:10:38] stderr:
[01:10:38] ------------------------------------------
[01:10:38] thread 'rustc' panicked at 'Found unstable fingerprints for NormalizeTyAfterErasingRegions(2e42b4c9fadc4b6c-d7f0ed43453b5699)', src/librustc/ty/query/plumbing.rs:528:9
[01:10:38] 
[01:10:38] error: internal compiler error: unexpected panic
[01:10:38] 
[01:10:38] note: the compiler unexpectedly panicked. this is a bug.
[01:10:38] note: the compiler unexpectedly panicked. this is a bug.
[01:10:38] 
[01:10:38] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:10:38] 
[01:10:38] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:10:38] 
[01:10:38] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath
[01:10:38] 
[01:10:38] ------------------------------------------
[01:10:38] 
[01:10:38] thread '[incremental] incremental/static_stable_hash/issue-49301.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
---
[01:10:38] test result: FAILED. 87 passed; 11 failed; 0 ignored; 0 measured; 0 filtered out
[01:10:38] 
[01:10:38] 
[01:10:38] 
[01:10:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:10:38] 
[01:10:38] 
[01:10:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:10:38] Build completed unsuccessfully in 0:11:24
[01:10:38] Build completed unsuccessfully in 0:11:24
[01:10:38] make: *** [check] Error 1
[01:10:38] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0723eb38
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Feb 15 14:01:01 UTC 2019
---
travis_time:end:00dd9cd0:start=1550239262596642536,finish=1550239262646080963,duration=49438427
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1b6a2d70
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6

