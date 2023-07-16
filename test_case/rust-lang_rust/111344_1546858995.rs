plain


pass test got exit status: 101, but expected 0

A bug in `ui_test` occurred: test panicked: stderr:
{"message":"Miri does not support optimizations. If you have enabled optimizations by selecting a Cargo profile (such as --release) which changes other profile settings such as whether debug assertions and overflow checks are enabled, those settings are still applied.","code":null,"level":"warning","spans":[],"children":[],"rendered":"warning: Miri does not support optimizations. If you have enabled optimizations by selecting a Cargo profile (such as --release) which changes other profile settings such as whether debug assertions and overflow checks are enabled, those settings are still applied.\n\n"}
{"message":"You have explicitly enabled MIR optimizations, overriding Miri's default which is to completely disable them. Any optimizations may hide UB that Miri would otherwise detect, and it is not necessarily possible to predict what kind of UB will be missed. If you are enabling optimizations to make Miri run faster, we advise using cfg(miri) to shrink your workload instead. The performance benefit of enabling MIR optimizations is usually marginal at best.","code":null,"level":"warning","spans":[],"children":[],"rendered":"warning: You have explicitly enabled MIR optimizations, overriding Miri's default which is to completely disable them. Any optimizations may hide UB that Miri would otherwise detect, and it is not necessarily possible to predict what kind of UB will be missed. If you are enabling optimizations to make Miri run faster, we advise using cfg(miri) to shrink your workload instead. The performance benefit of enabling MIR optimizations is usually marginal at best.\n\n"}
thread 'main' panicked at 'assertion failed: return_fn_ptr(f) != f', tests/pass/function_pointers.rs:78:5

stdout:


---
test result: FAIL. 1 tests failed, 293 tests passed, 5 ignored, 0 filtered out
Error: 
   0: tests failed

Backtrace omitted. Run with RUST_BACKTRACE=1 environment variable to display it.
Run with RUST_BACKTRACE=full to include source snippets.

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/compiletest-fcd82489d5eb52c4 --quiet tests/pass tests/panic -- --quiet` (exit status: 1)
