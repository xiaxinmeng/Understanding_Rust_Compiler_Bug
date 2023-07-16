plain
travis_time:end:008f1d62:start=1550309103515726172,finish=1550309107105995714,duration=3590269542
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
[01:11:40] 
[01:11:40] running 98 tests
[01:11:55] ..................F..F.....FFF.F.FFF.F...F.FF.......F.............................................
[01:11:55] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:11:55] 
[01:11:55] ---- [incremental] incremental/hashes/call_expressions.rs stdout ----
[01:11:55] 
[01:11:55] 
[01:11:55] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:11:55] status: exit code: 1
[01:11:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/call_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions/call_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions/auxiliary"
[01:11:55] ------------------------------------------
[01:11:55] 
[01:11:55] ------------------------------------------
[01:11:55] stderr:
[01:11:55] stderr:
[01:11:55] ------------------------------------------
[01:11:55] {"message":"`Hir(change_to_ufcs)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/call_expressions.rs","byte_start":3917,"byte_end":4000,"line_start":156,"line_end":159,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub fn change_to_ufcs() {","highlight_start":1,"highlight_end":26},{"text":"    let s = Struct;","highlight_start":1,"highlight_end":20},{"text":"    Struct::method1(&s, 'x', true);","highlight_start":1,"highlight_end":36},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `Hir(change_to_ufcs)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/call_expressions.rs:156:1\n   |\nLL | / pub fn change_to_ufcs() {\nLL | |     let s = Struct;\nLL | |     Struct::method1(&s, 'x', true);\nLL | | }\n   | |_^\n\n"}
[01:11:55] 
[01:11:55] ------------------------------------------
[01:11:55] 
[01:11:55] thread '[incremental] incremental/hashes/call_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:11:55] thread '[incremental] incremental/hashes/call_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:11:55] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:11:55] 
[01:11:55] ---- [incremental] incremental/hashes/closure_expressions.rs stdout ----
[01:11:55] 
[01:11:55] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:11:55] status: exit code: 1
[01:11:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/closure_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/closure_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/auxiliary"
[01:11:55] ------------------------------------------
[01:11:55] 
[01:11:55] ------------------------------------------
[01:11:55] stderr:
[01:11:55] stderr:
[01:11:55] ------------------------------------------
[01:11:55] {"message":"`Hir(add_parameter)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/closure_expressions.rs","byte_start":1108,"byte_end":1180,"line_start":42,"line_end":45,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub fn add_parameter() {","highlight_start":1,"highlight_end":25},{"text":"    let x = 0u32;","highlight_start":1,"highlight_end":18},{"text":"    let _ = |x: u32| x + 1;","highlight_start":1,"highlight_end":28},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `Hir(add_parameter)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/closure_expressions.rs:42:1\n   |\nLL | / pub fn add_parameter() {\nLL | |     let x = 0u32;\nLL | |     let _ = |x: u32| x + 1;\nLL | | }\n   | |_^\n\n"}
[01:11:55] {"message":"`Hir(change_parameter_pattern)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/closure_expressions.rs","byte_start":1464,"byte_end":1527,"line_start":58,"line_end":60,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub fn change_parameter_pattern() {","highlight_start":1,"highlight_end":36},{"text":"    let _ = |&x: &u32| x;","highlight_start":1,"highlight_end":26},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `Hir(change_parameter_pattern)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/closure_expressions.rs:58:1\n   |\nLL | / pub fn change_parameter_pattern() {\nLL | |     let _ = |&x: &u32| x;\nLL | | }\n   | |_^\n\n"}
[01:11:55] {"message":"`Hir(add_type_ascription_to_parameter)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/closure_expressions.rs","byte_start":2136,"byte_end":2247,"line_start":89,"line_end":92,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub fn add_type_ascription_to_parameter() {","highlight_start":1,"highlight_end":44},{"text":"    let closure = |x: u32| x + 1u32;","highlight_start":1,"highlight_end":37},{"text":"    let _: u32 = closure(1);","highlight_start":1,"highlight_end":29},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `Hir(add_type_ascription_to_parameter)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/closure_expressions.rs:89:1\n   |\nLL | / pub fn add_type_ascription_to_parameter() {\nLL | |     let closure = |x: u32| x + 1u32;\nLL | |     let _: u32 = closure(1);\nLL | | }\n   | |_^\n\n"}
[01:11:55] 
[01:11:55] ------------------------------------------
[01:11:55] 
[01:11:55] thread '[incremental] incremental/hashes/closure_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:11:55] thread '[incremental] incremental/hashes/closure_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:11:55] 
[01:11:55] ---- [incremental] incremental/hashes/if_expressions.rs stdout ----
[01:11:55] 
[01:11:55] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:11:55] status: exit code: 1
[01:11:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/if_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions/if_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions/auxiliary"
[01:11:55] ------------------------------------------
[01:11:55] 
[01:11:55] ------------------------------------------
[01:11:55] stderr:
[01:11:55] stderr:
[01:11:55] ------------------------------------------
[01:11:55] {"message":"`Hir(change_condition)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/if_expressions.rs","byte_start":822,"byte_end":914,"line_start":30,"line_end":36,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub fn change_condition(x: bool) -> u32 {","highlight_start":1,"highlight_end":42},{"text":"    if !x {","highlight_start":1,"highlight_end":12},{"text":"        return 1","highlight_start":1,"highlight_end":17},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"","highlight_start":1,"highlight_end":1},{"text":"    return 0","highlight_start":1,"highlight_end":13},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `Hir(change_condition)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/if_expressions.rs:30:1\n   |\nLL | / pub fn change_condition(x: bool) -> u32 {\nLL | |     if !x {\nLL | |         return 1\nLL | |     }\nLL | |\nLL | |     return 0\nLL | | }\n   | |_^\n\n"}
[01:11:55] {"message":"`Hir(add_else_branch)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/if_expressions.rs","byte_start":2046,"byte_end":2166,"line_start":99,"line_end":108,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub fn add_else_branch(x: bool) -> u32 {","highlight_start":1,"highlight_end":41},{"text":"    let mut ret = 1;","highlight_start":1,"highlight_end":21},{"text":"","highlight_start":1,"highlight_end":1},{"text":"    if x {","highlight_start":1,"highlight_end":11},{"text":"        ret = 2;","highlight_start":1,"highlight_end":17},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"","highlight_start":1,"highlight_end":1},{"text":"    ret","highlight_start":1,"highlight_end":8},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `Hir(add_else_branch)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/if_expressions.rs:99:1\n   |\nLL | / pub fn add_else_branch(x: bool) -> u32 {\nLL | |     let mut ret = 1;\nLL | |\nLL | |     if x {\n...  |\nLL | |     ret\nLL | | }\n   | |_^\n\n"}
[01:11:55] {"message":"`Hir(change_then_branch_if_let)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/if_expressions.rs","byte_start":2971,"byte_end":3089,"line_start":148,"line_end":154,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub fn change_then_branch_if_let(x: Option<u32>) -> u32 {","highlight_start":1,"highlight_end":58},{"text":"    if let Some(x) = x {","highlight_start":1,"highlight_end":25},{"text":"        return x + 1","highlight_start":1,"highlight_end":21},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"","highlight_start":1,"highlight_end":1},{"text":"    0","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `Hir(change_then_branch_if_let)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/if_expressions.rs:148:1\n   |\nLL | / pub fn change_then_branch_if_let(x: Option<u32>) -> u32 {\nLL | |     if let Some(x) = x {\nLL | |         return x + 1\nLL | |     }\nLL | |\nLL | |     0\nLL | | }\n   | |_^\n\n"}
[01:11:55] {"message":"`Hir(add_else_branch_if_let)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/if_expressions.rs","byte_start":3901,"byte_end":4049,"line_start":196,"line_end":205,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub fn add_else_branch_if_let(x: Option<u32>) -> u32 {","highlight_start":1,"highlight_end":55},{"text":"    let mut ret = 1;","highlight_start":1,"highlight_end":21},{"text":"","highlight_start":1,"highlight_end":1},{"text":"    if let Some(x) = x {","highlight_start":1,"highlight_end":25},{"text":"        ret = x;","highlight_start":1,"highlight_end":17},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"","highlight_start":1,"highlight_end":1},{"text":"    ret","highlight_start":1,"highlight_end":8},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `Hir(add_else_branch_if_let)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/if_expressions.rs:196:1\n   |\nLL | / pub fn add_else_branch_if_let(x: Option<u32>) -> u32 {\nLL | |     let mut ret = 1;\nLL | |\nLL | |     if let Some(x) = x {\n...  |\nLL | |     ret\nLL | | }\n   | |_^\n\n"}
[01:11:55] 
[01:11:55] ------------------------------------------
[01:11:55] 
[01:11:55] thread '[incremental] incremental/hashes/if_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:11:55] thread '[incremental] incremental/hashes/if_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:11:55] 
[01:11:55] ---- [incremental] incremental/hashes/indexing_expressions.rs stdout ----
[01:11:55] 
[01:11:55] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:11:55] status: exit code: 1
[01:11:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/indexing_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/indexing_expressions/indexing_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/indexing_expressions/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/indexing_expressions/auxiliary"
[01:11:55] ------------------------------------------
[01:11:55] 
[01:11:55] ------------------------------------------
[01:11:55] stderr:
[01:11:55] stderr:
[01:11:55] ------------------------------------------
[01:11:55] {"message":"`Hir(add_lower_bound)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/indexing_expressions.rs","byte_start":2147,"byte_end":2211,"line_start":78,"line_end":80,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn add_lower_bound(slice: &[u32]) -> &[u32] {","highlight_start":1,"highlight_end":46},{"text":"    &slice[3..4]","highlight_start":1,"highlight_end":17},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `Hir(add_lower_bound)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/indexing_expressions.rs:78:1\n   |\nLL | / fn add_lower_bound(slice: &[u32]) -> &[u32] {\nLL | |     &slice[3..4]\nLL | | }\n   | |_^\n\n"}
[01:11:55] {"message":"`Hir(add_upper_bound)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/indexing_expressions.rs","byte_start":2572,"byte_end":2636,"line_start":95,"line_end":97,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn add_upper_bound(slice: &[u32]) -> &[u32] {","highlight_start":1,"highlight_end":46},{"text":"    &slice[3..7]","highlight_start":1,"highlight_end":17},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `Hir(add_upper_bound)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/indexing_expressions.rs:95:1\n   |\nLL | / fn add_upper_bound(slice: &[u32]) -> &[u32] {\nLL | |     &slice[3..7]\nLL | | }\n   | |_^\n\n"}
[01:11:55] 
[01:11:55] ------------------------------------------
[01:11:55] 
[01:11:55] thread '[incremental] incremental/hashes/indexing_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:11:55] thread '[incremental] incremental/hashes/indexing_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:11:55] 
[01:11:55] ---- [incremental] incremental/hashes/for_loops.rs stdout ----
[01:11:55] 
[01:11:55] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:11:55] status: exit code: 1
[01:11:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/for_loops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/for_loops.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/auxiliary"
[01:11:55] ------------------------------------------
[01:11:55] 
[01:11:55] ------------------------------------------
[01:11:55] stderr:
[01:11:55] stderr:
[01:11:55] ------------------------------------------
[01:11:55] {"message":"`Hir(change_iteration_variable_pattern)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/for_loops.rs","byte_start":1759,"byte_end":1890,"line_start":76,"line_end":82,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub fn change_iteration_variable_pattern() {","highlight_start":1,"highlight_end":45},{"text":"    let mut _x = 0;","highlight_start":1,"highlight_end":20},{"text":"    for &_i in &[0, 1, 2] {","highlight_start":1,"highlight_end":28},{"text":"        _x = 1;","highlight_start":1,"highlight_end":16},{"text":"        break;","highlight_start":1,"highlight_end":15},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `Hir(change_iteration_variable_pattern)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/for_loops.rs:76:1\n   |\nLL | / pub fn change_iteration_variable_pattern() {\nLL | |     let mut _x = 0;\nLL | |     for &_i in &[0, 1, 2] {\nLL | |         _x = 1;\nLL | |         break;\nLL | |     }\nLL | | }\n   | |_^\n\n"}
[01:11:55] {"message":"`Hir(add_break)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/for_loops.rs","byte_start":2654,"byte_end":2753,"line_start":121,"line_end":127,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub fn add_break() {","highlight_start":1,"highlight_end":21},{"text":"    let mut _x = 0;","highlight_start":1,"highlight_end":20},{"text":"    for _ in 0..1 {","highlight_start":1,"highlight_end":20},{"text":"        _x = 1;","highlight_start":1,"highlight_end":16},{"text":"        break;","highlight_start":1,"highlight_end":15},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `Hir(add_break)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/for_loops.rs:121:1\n   |\nLL | / pub fn add_break() {\nLL | |     let mut _x = 0;\nLL | |     for _ in 0..1 {\nLL | |         _x = 1;\nLL | |         break;\nLL | |     }\nLL | | }\n   | |_^\n\n"}
[01:11:55] 
[01:11:55] ------------------------------------------
[01:11:55] 
[01:11:55] thread '[incremental] incremental/hashes/for_loops.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:11:55] thread '[incremental] incremental/hashes/for_loops.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:11:55] 
[01:11:55] ---- [incremental] incremental/hashes/let_expressions.rs stdout ----
[01:11:55] 
[01:11:55] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:11:55] status: exit code: 1
[01:11:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/let_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions/let_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions/auxiliary"
[01:11:55] ------------------------------------------
[01:11:55] 
[01:11:55] ------------------------------------------
[01:11:55] stderr:
[01:11:55] stderr:
[01:11:55] ------------------------------------------
[01:11:55] {"message":"`Hir(add_type)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/let_expressions.rs","byte_start":1086,"byte_end":1131,"line_start":43,"line_end":45,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub fn add_type() {","highlight_start":1,"highlight_end":20},{"text":"    let _x: u32 = 2u32;","highlight_start":1,"highlight_end":24},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `Hir(add_type)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/let_expressions.rs:43:1\n   |\nLL | / pub fn add_type() {\nLL | |     let _x: u32 = 2u32;\nLL | | }\n   | |_^\n\n"}
[01:11:55] {"message":"`Hir(change_simple_binding_to_pattern)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/let_expressions.rs","byte_start":2496,"byte_end":2572,"line_start":107,"line_end":109,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub fn change_simple_binding_to_pattern() {","highlight_start":1,"highlight_end":44},{"text":"    let (_a, _b) = (0u8, 'x');","highlight_start":1,"highlight_end":31},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `Hir(change_simple_binding_to_pattern)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/let_expressions.rs:107:1\n   |\nLL | / pub fn change_simple_binding_to_pattern() {\nLL | |     let (_a, _b) = (0u8, 'x');\nLL | | }\n   | |_^\n\n"}
[01:11:55] {"message":"`Hir(add_amp_in_pattern)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/let_expressions.rs","byte_start":3593,"byte_end":3657,"line_start":155,"line_end":157,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub fn add_amp_in_pattern() {","highlight_start":1,"highlight_end":30},{"text":"    let (&_a, _b) = (&1u8, 'y');","highlight_start":1,"highlight_end":33},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `Hir(add_amp_in_pattern)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/let_expressions.rs:155:1\n   |\nLL | / pub fn add_amp_in_pattern() {\nLL | |     let (&_a, _b) = (&1u8, 'y');\nLL | | }\n   | |_^\n\n"}
[01:11:55] {"message":"`Hir(add_initializer)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/let_expressions.rs","byte_start":4348,"byte_end":4400,"line_start":187,"line_end":189,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub fn add_initializer() {","highlight_start":1,"highlight_end":27},{"text":"    let _x: i16 = 3i16;","highlight_start":1,"highlight_end":24},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `Hir(add_initializer)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/let_expressions.rs:187:1\n   |\nLL | / pub fn add_initializer() {\nLL | |     let _x: i16 = 3i16;\nLL | | }\n   | |_^\n\n"}
[01:11:55] 
[01:11:55] ------------------------------------------
[01:11:55] 
[01:11:55] thread '[incremental] incremental/hashes/let_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:11:55] thread '[incremental] incremental/hashes/let_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:11:55] 
[01:11:55] ---- [incremental] incremental/hashes/inherent_impls.rs stdout ----
[01:11:55] 
[01:11:55] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:11:55] status: exit code: 1
[01:11:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/inherent_impls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls/inherent_impls.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls/auxiliary"
[01:11:55] ------------------------------------------
[01:11:55] 
[01:11:55] ------------------------------------------
[01:11:55] stderr:
[01:11:55] stderr:
[01:11:55] ------------------------------------------
[01:11:55] {"message":"`Hir(Foo::method_body)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/inherent_impls.rs","byte_start":1272,"byte_end":1335,"line_start":47,"line_end":49,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn method_body() {","highlight_start":5,"highlight_end":27},{"text":"        println!(\"Hello, world!\");","highlight_start":1,"highlight_end":35},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `Hir(Foo::method_body)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/inherent_impls.rs:47:5\n   |\nLL | /     pub fn method_body() {\nLL | |         println!(\"Hello, world!\");\nLL | |     }\n   | |_____^\n\n"}
[01:11:55] {"message":"`Hir(Foo::method_body_inlined)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/inherent_impls.rs","byte_start":1789,"byte_end":1860,"line_start":69,"line_end":71,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn method_body_inlined() {","highlight_start":5,"highlight_end":35},{"text":"        println!(\"Hello, world!\");","highlight_start":1,"highlight_end":35},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `Hir(Foo::method_body_inlined)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/inherent_impls.rs:69:5\n   |\nLL | /     pub fn method_body_inlined() {\nLL | |         println!(\"Hello, world!\");\nLL | |     }\n   | |_____^\n\n"}
[01:11:55] 
[01:11:55] ------------------------------------------
[01:11:55] 
[01:11:55] thread '[incremental] incremental/hashes/inherent_impls.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:11:55] thread '[incremental] incremental/hashes/inherent_impls.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:11:55] 
[01:11:55] ---- [incremental] incremental/hashes/loop_expressions.rs stdout ----
[01:11:55] 
[01:11:55] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:11:55] status: exit code: 1
[01:11:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/loop_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/loop_expressions/loop_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/loop_expressions/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/loop_expressions/auxiliary"
[01:11:55] ------------------------------------------
[01:11:55] 
[01:11:55] ------------------------------------------
[01:11:55] stderr:
[01:11:55] stderr:
[01:11:55] ------------------------------------------
[01:11:55] {"message":"`Hir(add_break)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/loop_expressions.rs","byte_start":1223,"byte_end":1313,"line_start":52,"line_end":58,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub fn add_break() {","highlight_start":1,"highlight_end":21},{"text":"    let mut _x = 0;","highlight_start":1,"highlight_end":20},{"text":"    loop {","highlight_start":1,"highlight_end":11},{"text":"        _x = 1;","highlight_start":1,"highlight_end":16},{"text":"        break;","highlight_start":1,"highlight_end":15},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `Hir(add_break)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/loop_expressions.rs:52:1\n   |\nLL | / pub fn add_break() {\nLL | |     let mut _x = 0;\nLL | |     loop {\nLL | |         _x = 1;\nLL | |         break;\nLL | |     }\nLL | | }\n   | |_^\n\n"}
[01:11:55] 
[01:11:55] ------------------------------------------
[01:11:55] 
[01:11:55] thread '[incremental] incremental/hashes/loop_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:11:55] thread '[incremental] incremental/hashes/loop_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:11:55] 
[01:11:55] ---- [incremental] incremental/hashes/match_expressions.rs stdout ----
[01:11:55] 
[01:11:55] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:11:55] status: exit code: 1
[01:11:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/match_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/match_expressions/match_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/match_expressions/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/match_expressions/auxiliary"
[01:11:55] ------------------------------------------
[01:11:55] 
[01:11:55] ------------------------------------------
[01:11:55] stderr:
[01:11:55] stderr:
[01:11:55] ------------------------------------------
[01:11:55] {"message":"`Hir(add_arm)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/match_expressions.rs","byte_start":841,"byte_end":960,"line_start":31,"line_end":38,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub fn add_arm(x: u32) -> u32 {","highlight_start":1,"highlight_end":32},{"text":"    match x {","highlight_start":1,"highlight_end":14},{"text":"        0 => 0,","highlight_start":1,"highlight_end":16},{"text":"        1 => 1,","highlight_start":1,"highlight_end":16},{"text":"        2 => 2,","highlight_start":1,"highlight_end":16},{"text":"        _ => 100,","highlight_start":1,"highlight_end":18},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `Hir(add_arm)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/match_expressions.rs:31:1\n   |\nLL | / pub fn add_arm(x: u32) -> u32 {\nLL | |     match x {\nLL | |         0 => 0,\nLL | |         1 => 1,\n...  |\nLL | |     }\nLL | | }\n   | |_^\n\n"}
[01:11:55] {"message":"`Hir(add_guard_clause)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/match_expressions.rs","byte_start":1774,"byte_end":1900,"line_start":80,"line_end":86,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub fn add_guard_clause(x: u32, y: bool) -> u32 {","highlight_start":1,"highlight_end":50},{"text":"    match x {","highlight_start":1,"highlight_end":14},{"text":"        0 => 0,","highlight_start":1,"highlight_end":16},{"text":"        1 if y => 1,","highlight_start":1,"highlight_end":21},{"text":"        _ => 100,","highlight_start":1,"highlight_end":18},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `Hir(add_guard_clause)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/match_expressions.rs:80:1\n   |\nLL | / pub fn add_guard_clause(x: u32, y: bool) -> u32 {\nLL | |     match x {\nLL | |         0 => 0,\nLL | |         1 if y => 1,\nLL | |         _ => 100,\nLL | |     }\nLL | | }\n   | |_^\n\n"}
[01:11:55] {"message":"`Hir(change_guard_clause)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/match_expressions.rs","byte_start":2269,"byte_end":2399,"line_start":104,"line_end":110,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub fn change_guard_clause(x: u32, y: bool) -> u32 {","highlight_start":1,"highlight_end":53},{"text":"    match x {","highlight_start":1,"highlight_end":14},{"text":"        0 => 0,","highlight_start":1,"highlight_end":16},{"text":"        1 if !y => 1,","highlight_start":1,"highlight_end":22},{"text":"        _ => 100,","highlight_start":1,"highlight_end":18},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `Hir(change_guard_clause)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/match_expressions.rs:104:1\n   |\nLL | / pub fn change_guard_clause(x: u32, y: bool) -> u32 {\nLL | |     match x {\nLL | |         0 => 0,\nLL | |         1 if !y => 1,\nLL | |         _ => 100,\nLL | |     }\nLL | | }\n   | |_^\n\n"}
[01:11:55] {"message":"`Hir(add_at_binding)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/match_expressions.rs","byte_start":2744,"byte_end":2856,"line_start":128,"line_end":134,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub fn add_at_binding(x: u32) -> u32 {","highlight_start":1,"highlight_end":39},{"text":"    match x {","highlight_start":1,"highlight_end":14},{"text":"        0 => 0,","highlight_start":1,"highlight_end":16},{"text":"        1 => 1,","highlight_start":1,"highlight_end":16},{"text":"        x @ _ => x,","highlight_start":1,"highlight_end":20},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `Hir(add_at_binding)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/match_expressions.rs:128:1\n   |\nLL | / pub fn add_at_binding(x: u32) -> u32 {\nLL | |     match x {\nLL | |         0 => 0,\nLL | |         1 => 1,\nLL | |         x @ _ => x,\nLL | |     }\nLL | | }\n   | |_^\n\n"}
[01:11:55] {"message":"`Hir(change_simple_name_to_pattern)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/match_expressions.rs","byte_start":3684,"byte_end":3810,"line_start":175,"line_end":180,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub fn change_simple_name_to_pattern(x: u32) -> u32 {","highlight_start":1,"highlight_end":54},{"text":"    match (x, x & 1) {","highlight_start":1,"highlight_end":23},{"text":"        (0, 0) => 0,","highlight_start":1,"highlight_end":21},{"text":"        (x, y) => 1,","highlight_start":1,"highlight_end":21},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `Hir(change_simple_name_to_pattern)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/match_expressions.rs:175:1\n   |\nLL | / pub fn change_simple_name_to_pattern(x: u32) -> u32 {\nLL | |     match (x, x & 1) {\nLL | |         (0, 0) => 0,\nLL | |         (x, y) => 1,\nLL | |     }\nLL | | }\n   | |_^\n\n"}
[01:11:55] {"message":"`Hir(add_amp_to_binding_in_pattern)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/match_expressions.rs","byte_start":5631,"byte_end":5754,"line_start":265,"line_end":270,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub fn add_amp_to_binding_in_pattern(x: u32) -> u32 {","highlight_start":1,"highlight_end":54},{"text":"    match (&x, x & 1) {","highlight_start":1,"highlight_end":24},{"text":"        (&a, 0) => 0,","highlight_start":1,"highlight_end":22},{"text":"        _ => 1,","highlight_start":1,"highlight_end":16},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `Hir(add_amp_to_binding_in_pattern)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/match_expressions.rs:265:1\n   |\nLL | / pub fn add_amp_to_binding_in_pattern(x: u32) -> u32 {\nLL | |     match (&x, x & 1) {\nLL | |         (&a, 0) => 0,\nLL | |         _ => 1,\nLL | |     }\nLL | | }\n   | |_^\n\n"}
[01:11:55] {"message":"`Hir(add_alternative_to_arm)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/match_expressions.rs","byte_start":6553,"byte_end":6673,"line_start":312,"line_end":318,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub fn add_alternative_to_arm(x: u32) -> u32 {","highlight_start":1,"highlight_end":47},{"text":"    match x {","highlight_start":1,"highlight_end":14},{"text":"        0 | 7 => 0,","highlight_start":1,"highlight_end":20},{"text":"        1 => 3,","highlight_start":1,"highlight_end":16},{"text":"        _ => 2,","highlight_start":1,"highlight_end":16},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `Hir(add_alternative_to_arm)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/match_expressions.rs:312:1\n   |\nLL | / pub fn add_alternative_to_arm(x: u32) -> u32 {\nLL | |     match x {\nLL | |         0 | 7 => 0,\nLL | |         1 => 3,\nLL | |         _ => 2,\nLL | |     }\nLL | | }\n   | |_^\n\n"}
[01:11:55] 
[01:11:55] ------------------------------------------
[01:11:55] 
[01:11:55] thread '[incremental] incremental/hashes/match_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:11:55] thread '[incremental] incremental/hashes/match_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:11:55] 
[01:11:55] ---- [incremental] incremental/hashes/struct_constructors.rs stdout ----
[01:11:55] 
[01:11:55] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:11:55] status: exit code: 1
[01:11:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/struct_constructors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/struct_constructors/struct_constructors.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/struct_constructors/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/struct_constructors/auxiliary"
[01:11:55] ------------------------------------------
[01:11:55] 
[01:11:55] ------------------------------------------
[01:11:55] stderr:
[01:11:55] stderr:
[01:11:55] ------------------------------------------
[01:11:55] {"message":"`Hir(add_field_regular_struct)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/struct_constructors.rs","byte_start":1965,"byte_end":2176,"line_start":87,"line_end":99,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub fn add_field_regular_struct() -> RegularStruct {","highlight_start":1,"highlight_end":53},{"text":"    let struct1 = RegularStruct {","highlight_start":1,"highlight_end":34},{"text":"        x: 3,","highlight_start":1,"highlight_end":14},{"text":"        y: 4,","highlight_start":1,"highlight_end":14},{"text":"        z: 5,","highlight_start":1,"highlight_end":14},{"text":"    };","highlight_start":1,"highlight_end":7},{"text":"","highlight_start":1,"highlight_end":1},{"text":"    RegularStruct {","highlight_start":1,"highlight_end":20},{"text":"        x: 7,","highlight_start":1,"highlight_end":14},{"text":"        y: 8,","highlight_start":1,"highlight_end":14},{"text":"        .. struct1","highlight_start":1,"highlight_end":19},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `Hir(add_field_regular_struct)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/struct_constructors.rs:87:1\n   |\nLL | / pub fn add_field_regular_struct() -> RegularStruct {\nLL | |     let struct1 = RegularStruct {\nLL | |         x: 3,\nLL | |         y: 4,\n...  |\nLL | |     }\nLL | | }\n   | |_^\n\n"}
[01:11:55] 
[01:11:55] ------------------------------------------
[01:11:55] 
[01:11:55] thread '[incremental] incremental/hashes/struct_constructors.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:11:55] thread '[incremental] incremental/hashes/struct_constructors.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:11:55] 
[01:11:55] ---- [incremental] incremental/hashes/trait_impls.rs stdout ----
[01:11:55] 
[01:11:55] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:11:55] status: exit code: 1
[01:11:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/trait_impls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_impls/trait_impls.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_impls/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_impls/auxiliary"
[01:11:55] ------------------------------------------
[01:11:55] 
[01:11:55] ------------------------------------------
[01:11:55] stderr:
[01:11:55] stderr:
[01:11:55] ------------------------------------------
[01:11:55] {"message":"`Hir(<Foo as ChangeMethodBodyTrait>::method_name)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/trait_impls.rs","byte_start":1828,"byte_end":1863,"line_start":69,"line_end":71,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    fn method_name() {","highlight_start":5,"highlight_end":23},{"text":"        ()","highlight_start":1,"highlight_end":11},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `Hir(<Foo as ChangeMethodBodyTrait>::method_name)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/trait_impls.rs:69:5\n   |\nLL | /     fn method_name() {\nLL | |         ()\nLL | |     }\n   | |_____^\n\n"}
[01:11:55] {"message":"`Hir(<Foo as ChangeMethodBodyTraitInlined>::method_name)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/trait_impls.rs","byte_start":2536,"byte_end":2577,"line_start":97,"line_end":99,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    fn method_name() {","highlight_start":5,"highlight_end":23},{"text":"        panic!()","highlight_start":1,"highlight_end":17},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `Hir(<Foo as ChangeMethodBodyTraitInlined>::method_name)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/trait_impls.rs:97:5\n   |\nLL | /     fn method_name() {\nLL | |         panic!()\nLL | |     }\n   | |_____^\n\n"}
[01:11:55] 
[01:11:55] ------------------------------------------
[01:11:55] 
[01:11:55] thread '[incremental] incremental/hashes/trait_impls.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:11:55] thread '[incremental] incremental/hashes/trait_impls.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:11:55] 
[01:11:55] ---- [incremental] incremental/hashes/while_let_loops.rs stdout ----
[01:11:55] 
[01:11:55] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:11:55] status: exit code: 1
[01:11:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/while_let_loops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_let_loops/while_let_loops.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_let_loops/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_let_loops/auxiliary"
[01:11:55] ------------------------------------------
[01:11:55] 
[01:11:55] ------------------------------------------
[01:11:55] stderr:
[01:11:55] stderr:
[01:11:55] ------------------------------------------
[01:11:55] {"message":"`Hir(add_break)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/while_let_loops.rs","byte_start":1769,"byte_end":1882,"line_start":75,"line_end":81,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub fn add_break() {","highlight_start":1,"highlight_end":21},{"text":"    let mut _x = 0;","highlight_start":1,"highlight_end":20},{"text":"    while let Some(0u32) = None {","highlight_start":1,"highlight_end":34},{"text":"        _x = 1;","highlight_start":1,"highlight_end":16},{"text":"        break;","highlight_start":1,"highlight_end":15},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `Hir(add_break)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/while_let_loops.rs:75:1\n   |\nLL | / pub fn add_break() {\nLL | |     let mut _x = 0;\nLL | |     while let Some(0u32) = None {\nLL | |         _x = 1;\nLL | |         break;\nLL | |     }\nLL | | }\n   | |_^\n\n"}
[01:11:55] 
[01:11:55] ------------------------------------------
[01:11:55] 
[01:11:55] thread '[incremental] incremental/hashes/while_let_loops.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:11:55] thread '[incremental] incremental/hashes/while_let_loops.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:11:55] 
[01:11:55] ---- [incremental] incremental/hashes/while_loops.rs stdout ----
[01:11:55] 
[01:11:55] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:11:55] status: exit code: 1
[01:11:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/while_loops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_loops/while_loops.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_loops/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_loops/auxiliary"
[01:11:55] ------------------------------------------
[01:11:55] 
[01:11:55] ------------------------------------------
[01:11:55] stderr:
[01:11:55] stderr:
[01:11:55] ------------------------------------------
[01:11:55] {"message":"`Hir(add_break)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/while_loops.rs","byte_start":1681,"byte_end":1777,"line_start":75,"line_end":81,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub fn add_break() {","highlight_start":1,"highlight_end":21},{"text":"    let mut _x = 0;","highlight_start":1,"highlight_end":20},{"text":"    while true {","highlight_start":1,"highlight_end":17},{"text":"        _x = 1;","highlight_start":1,"highlight_end":16},{"text":"        break;","highlight_start":1,"highlight_end":15},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `Hir(add_break)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/while_loops.rs:75:1\n   |\nLL | / pub fn add_break() {\nLL | |     let mut _x = 0;\nLL | |     while true {\nLL | |         _x = 1;\nLL | |         break;\nLL | |     }\nLL | | }\n   | |_^\n\n"}
[01:11:55] 
[01:11:55] ------------------------------------------
[01:11:55] 
[01:11:55] thread '[incremental] incremental/hashes/while_loops.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:11:55] thread '[incremental] incremental/hashes/while_loops.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:11:55] 
[01:11:55] ---- [incremental] incremental/issue-38222.rs stdout ----
[01:11:55] 
[01:11:55] error in revision `rpass2`: compilation failed!
[01:11:55] status: exit code: 1
[01:11:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-38222.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-38222/issue-38222.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-38222/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-C" "debuginfo=1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-38222/auxiliary"
[01:11:55] ------------------------------------------
[01:11:55] 
[01:11:55] ------------------------------------------
[01:11:55] stderr:
[01:11:55] stderr:
[01:11:55] ------------------------------------------
[01:11:55] {"message":"CGU-reuse for `issue_38222-mod1` is `No` but should be at least `PreLto`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/issue-38222.rs","byte_start":180,"byte_end":247,"line_start":11,"line_end":11,"column_start":1,"column_end":68,"is_primary":true,"text":[{"text":"#![rustc_partition_reused(module=\"issue_38222-mod1\", cfg=\"rpass2\")]","highlight_start":1,"highlight_end":68}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: CGU-reuse for `issue_38222-mod1` is `No` but should be at least `PreLto`\n  --> /checkout/src/test/incremental/issue-38222.rs:11:1\n   |\nLL | #![rustc_partition_reused(module=\"issue_38222-mod1\", cfg=\"rpass2\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:11:55] 
[01:11:55] ------------------------------------------
[01:11:55] 
[01:11:55] thread '[incremental] incremental/issue-38222.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
---
[01:11:55] test result: FAILED. 84 passed; 14 failed; 0 ignored; 0 measured; 0 filtered out
[01:11:55] 
[01:11:55] 
[01:11:55] 
[01:11:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:11:55] 
[01:11:55] 
[01:11:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:11:55] Build completed unsuccessfully in 0:11:21
[01:11:55] Build completed unsuccessfully in 0:11:21
[01:11:55] make: *** [check] Error 1
[01:11:55] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:25bb718f
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Feb 16 10:37:13 UTC 2019
