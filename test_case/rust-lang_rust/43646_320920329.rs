
[00:45:23] ---- [run-pass] run-pass/specialization/assoc-ty-graph-cycle.rs stdout ----
[00:45:23] 	
[00:45:23] error: compilation failed
[00:45:23] status: exit code: 101
[00:45:23] command: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /checkout/src/test/run-pass/specialization/assoc-ty-graph-cycle.rs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass --target=x86_64-unknown-linux-gnu -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/specialization/assoc-ty-graph-cycle.stage2-x86_64-unknown-linux-gnu.run-pass.libaux -C prefer-dynamic -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/specialization/assoc-ty-graph-cycle.stage2-x86_64-unknown-linux-gnu -Crpath -O -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[00:45:23] stdout:
[00:45:23] ------------------------------------------
[00:45:23] 
[00:45:23] ------------------------------------------
[00:45:23] stderr:
[00:45:23] ------------------------------------------
[00:45:23] error: internal compiler error: /checkout/src/librustc/traits/specialize/mod.rs:98: When translating substitutions for specialization, the expected specialization failed to hold
[00:45:23]
[00:45:23] note: the compiler unexpectedly panicked. this is a bug.
[00:45:23] 
[00:45:23] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:45:23] 
[00:45:23] note: rustc 1.21.0-dev (ae337e946 2017-08-04) running on x86_64-unknown-linux-gnu
[00:45:23] 
[00:45:23] thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc_errors/lib.rs:486:8
[00:45:23] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:45:23] 
[00:45:23] 
[00:45:23] ------------------------------------------
[00:45:23] 
[00:45:23] thread '[run-pass] run-pass/specialization/assoc-ty-graph-cycle.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2499:8
[00:45:23] note: Run with `RUST_BACKTRACE=1` for a backtrace.

[00:45:23] 

[00:45:23] 

[00:45:23] failures:

[00:45:23]     [run-pass] run-pass/specialization/assoc-ty-graph-cycle.rs
[00:45:23] 
