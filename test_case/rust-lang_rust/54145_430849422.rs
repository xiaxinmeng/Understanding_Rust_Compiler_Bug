plain
[00:44:37] .................................................................................................... 400/4622
[00:44:40] .................................................................................................... 500/4622
[00:44:44] .................i.................................................................................. 600/4622
[00:44:48] .................................................................................................... 700/4622
[00:44:53] ..................................................i...........i....FFFFF.FFF........................ 800/4622
[00:44:56] .....................................................................iiiii.......................... 900/4622
[00:45:01] .................................................................................................... 1100/4622
[00:45:04] .................................................................................................... 1200/4622
[00:45:06] .................................................................................................... 1300/4622
[00:45:08] .................................................................................................... 1400/4622
---
[00:46:55] 
[00:46:55] ------------------------------------------
[00:46:55] stderr:
[00:46:55] ------------------------------------------
[00:46:55] thread 'main' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', librustc/dep_graph/graph.rs:1080:9
[00:46:55] 
[00:46:55] error: internal compiler error: unexpected panic
[00:46:55] 
[00:46:55] note: the compiler unexpectedly panicked. this is a bug.
[00:46:55] note: the compiler unexpectedly panicked. this is a bug.
[00:46:55] 
[00:46:55] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:46:55] 
[00:46:55] note: rustc 1.31.0-dev running on x86_64-unknown-linux-gnu
[00:46:55] 
[00:46:55] note: compiler flags: -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[00:46:55] 
[00:46:55] ------------------------------------------
[00:46:55] 
[00:46:55] thread '[ui] ui/dep-graph/dep-graph-caller-callee.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3277:9
[00:46:55] thread '[ui] ui/dep-graph/dep-graph-caller-callee.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3277:9
[00:46:55] 
[00:46:55] ---- [ui] ui/dep-graph/dep-graph-struct-signature.rs stdout ----
[00:46:55] 
[00:46:55] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:46:55] status: exit code: 101
[00:46:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-struct-signature/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-struct-signature/auxiliary" "-A" "unused"
[00:46:55] ------------------------------------------
[00:46:55] 
[00:46:55] ------------------------------------------
[00:46:55] stderr:
[00:46:55] stderr:
[00:46:55] ------------------------------------------
[00:46:55] thread 'main' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', librustc/dep_graph/graph.rs:1080:9
[00:46:55] 
[00:46:55] error: internal compiler error: unexpected panic
[00:46:55] 
[00:46:55] note: the compiler unexpectedly panicked. this is a bug.
[00:46:55] note: the compiler unexpectedly panicked. this is a bug.
[00:46:55] 
[00:46:55] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:46:55] 
[00:46:55] note: rustc 1.31.0-dev running on x86_64-unknown-linux-gnu
[00:46:55] 
[00:46:55] note: compiler flags: -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[00:46:55] 
[00:46:55] ------------------------------------------
[00:46:55] 
[00:46:55] thread '[ui] ui/dep-graph/dep-graph-struct-signature.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3277:9
[00:46:55] thread '[ui] ui/dep-graph/dep-graph-struct-signature.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3277:9
[00:46:55] 
[00:46:55] ---- [ui] ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs stdout ----
[00:46:55] 
[00:46:55] error: Error: expected failure sta4-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-variance-alias/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-variance-alias/auxiliary" "-A" "unused"
[00:46:55] ------------------------------------------
[00:46:55] 
[00:46:55] ------------------------------------------
[00:46:55] stderr:
[00:46:55] stderr:
[00:46:55] ------------------------------------------
[00:46:55] thread 'main' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', librustc/dep_graph/graph.rs:1080:9
[00:46:55] 
[00:46:55] error: internal compiler error: unexpected panic
[00:46:55] 
[00:46:55] note: the compiler unexpectedly panicked. this is a bug.
[00:46:55] note: the compiler unexpectedly panicked. this is a bug.
[00:46:55] 
[00:46:55] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:46:55] 
[00:46:55] note: rustc 1.31.0-dev running on x86_64-unknown-linux-gnu
[00:46:55] 
[00:46:55] note: compiler flags: -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[00:46:55] 
[00:46:55] ------------------------------------------
[00:46:55] 
[00:46:55] thread '[ui] ui/dep-graph/dep-graph-variance-alias.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3277:9
