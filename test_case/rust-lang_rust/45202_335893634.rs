
[01:05:17] ---- [codegen-units] codegen-units/partitioning/local-inlining-but-not-all.rs stdout ----
[01:05:17] 	
[01:05:17] The following items were assigned to wrong codegen units:
[01:05:17] 
[01:05:17] fn local_inlining_but_not_all::inline[0]::inlined_function[0]
[01:05:17]   expected: local_inlining_but_not_all-inline[External] 
[01:05:17]   actual:   local_inlining_but_not_all-user1[Internal] local_inlining_but_not_all-user2[Internal] 
[01:05:17] 
[01:05:17] thread '[codegen-units] codegen-units/partitioning/local-inlining-but-not-all.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:1919:12
[01:05:17] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:05:17] 
[01:05:17] 
[01:05:17] failures:
[01:05:17]     [codegen-units] codegen-units/partitioning/local-inlining-but-not-all.rs
[01:05:17] 
[01:05:17] test result: [31mFAILED(B[m. 32 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out
