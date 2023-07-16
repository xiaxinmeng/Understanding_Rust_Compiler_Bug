
[00:51:43] failures:
[00:51:43] 
[00:51:43] ---- [codegen] codegen/vec-optimizes-away.rs stdout ----
[00:51:43] 	
[00:51:43] error: verification with 'FileCheck' failed
[00:51:43] status: exit code: 1
[00:51:43] command: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck -input-file=/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-optimizes-away.ll /checkout/src/test/codegen/vec-optimizes-away.rs
[00:51:43] stdout:
[00:51:43] ------------------------------------------
[00:51:43] 
[00:51:43] ------------------------------------------
[00:51:43] stderr:
[00:51:43] ------------------------------------------
[00:51:43] /checkout/src/test/codegen/vec-optimizes-away.rs:18:17: error: expected string not found in input
[00:51:43]  // CHECK-NEXT: bb{{.*}}:
[00:51:43]                 ^
[00:51:43] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-optimizes-away.ll:7:19: note: scanning from here
[00:51:43] define i32 @sum_me() unnamed_addr #0 personality i32 (...)* @rust_eh_personality {
[00:51:43]                   ^
[00:51:43] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-optimizes-away.ll:7:52: note: possible intended match here
[00:51:43] define i32 @sum_me() unnamed_addr #0 personality i32 (...)* @rust_eh_personality {
[00:51:43]                                                    ^
[00:51:43] 
[00:51:43] ------------------------------------------
[00:51:43] 
[00:51:43] thread '[codegen] codegen/vec-optimizes-away.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2499:8
[00:51:43] note: Run with `RUST_BACKTRACE=1` for a backtrace.
