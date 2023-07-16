
[00:50:00] ---- [codegen] codegen/issue-37945.rs stdout ----
[00:50:00] 	
[00:50:00] error: verification with 'FileCheck' failed
[00:50:00] status: exit code: 1
[00:50:00] command: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck -input-file=/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-37945.ll /checkout/src/test/codegen/issue-37945.rs
[00:50:00] stdout:
[00:50:00] ------------------------------------------
[00:50:00] 
[00:50:00] ------------------------------------------
[00:50:00] stderr:
[00:50:00] ------------------------------------------
[00:50:00] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-37945.ll:19:12: error: CHECK-NOT: string occurred!
[00:50:00]  %phitmp = icmp eq float* %2, null
[00:50:00]            ^
[00:50:00] /checkout/src/test/codegen/issue-37945.rs:23:15: note: CHECK-NOT: pattern specified here
[00:50:00] // CHECK-NOT: icmp eq float* {{.*}}, null
