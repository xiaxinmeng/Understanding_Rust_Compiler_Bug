plain
      Memory: 14 GB
      System Firmware Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMkJ2a7yCDA0
      Provisioning UDID: 4203018E-580F-C1B5-9525-B745CECA79EB

hw.ncpu: 3
hw.byteorder: 1234
---
failures:

---- [codegen] src/test/codegen/vec-calloc-2.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--input-file" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/codegen/vec-calloc-2/vec-calloc-2.ll" "/Users/runner/work/rust/rust/src/test/codegen/vec-calloc-2.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/Users/runner/work/rust/rust/src/test/codegen/vec-calloc-2.rs:22:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: __rust_alloc_zeroed(
               ^
/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/codegen/vec-calloc-2/vec-calloc-2.ll:359:18: note: found here
 %52 = call i8* @__rust_alloc_zeroed(i64 %48, i64 %spec.select.i.i.i.i.i) #8, !noalias !15


Input file: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/codegen/vec-calloc-2/vec-calloc-2.ll


-dump-input=help explains the following input dump.
Input was:
<<<<<<
        .
        .
        .
        .
      354: bb3.i.i22.i.i.i.i: ; preds = %bb11.i.i.i.i 
      355:  %51 = inttoptr i64 %spec.select.i.i.i.i.i to i8* 
      356:  br label %bb22.i.i.i.i 
      357:  
      358: bb2.i.i23.i.i.i.i: ; preds = %bb11.i.i.i.i 
      359:  %52 = call i8* @__rust_alloc_zeroed(i64 %48, i64 %spec.select.i.i.i.i.i) #8, !noalias !15 
not:22                      !~~~~~~~~~~~~~~~~~~~                                                       error: no match expected
      360:  br label %bb22.i.i.i.i 
      361:  
      362: bb22.i.i.i.i: ; preds = %bb2.i.i23.i.i.i.i, %bb3.i.i22.i.i.i.i 
      363:  %.sroa.0.0.i.i24.pn.i.i.i.i = phi i8* [ %51, %bb3.i.i22.i.i.i.i ], [ %52, %bb2.i.i23.i.i.i.i ] 
      364:  %53 = icmp eq i8* %.sroa.0.0.i.i24.pn.i.i.i.i, null 
        .
        .
>>>>>>
------------------------------------------
