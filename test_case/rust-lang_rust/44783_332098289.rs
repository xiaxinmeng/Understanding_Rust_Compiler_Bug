
[00:48:29] ---- [codegen] codegen/lto-removes-invokes.rs stdout ----
[00:48:29] 	
[00:48:29] error: verification with 'FileCheck' failed
[00:48:29] status: exit code: 2
[00:48:29] command: "/usr/lib/llvm-3.7/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/lto-removes-invokes.ll" "/checkout/src/test/codegen/lto-removes-invokes.rs"
[00:48:29] stdout:
[00:48:29] ------------------------------------------
[00:48:29] 
[00:48:29] ------------------------------------------
[00:48:29] stderr:
[00:48:29] ------------------------------------------
[00:48:29] Could not open input file '/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/lto-removes-invokes.ll': No such file or directory
[00:48:29] 
[00:48:29] ------------------------------------------
[00:48:29] 
[00:48:29] thread '[codegen] codegen/lto-removes-invokes.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2433:8
[00:48:29] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:29] 
[00:48:29] 
[00:48:29] failures:
[00:48:29]     [codegen] codegen/lto-removes-invokes.rs
[00:48:29] 
[00:48:29] test result: FAILED. 37 passed; 1 failed; 11 ignored; 0 measured; 0 filtered out
