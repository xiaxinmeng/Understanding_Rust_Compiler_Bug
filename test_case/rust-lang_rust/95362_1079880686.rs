plain
failures:

---- [codegen] codegen/vec-calloc-optimization.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-calloc-optimization/vec-calloc-optimization.ll" "/checkout/src/test/codegen/vec-calloc-optimization.rs" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/vec-calloc-optimization.rs:16:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: __rust_alloc(
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-calloc-optimization/vec-calloc-optimization.ll:52:17: note: found here
 %5 = call i8* @__rust_alloc(i64 7404, i64 2) #5, !noalias !26

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-calloc-optimization/vec-calloc-optimization.ll
Check file: /checkout/src/test/codegen/vec-calloc-optimization.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
        .
        .
        .
        .
       47:  %_4.i.i.i.i.i.i = load i16, i16* %2, align 8, !alias.scope !16, !noalias !23
       48:  %4 = icmp eq i16 %_4.i.i.i.i.i.i, 0
       49:  br i1 %4, label %bb3.i.i.i.i.1, label %bb4.i.i
       50: 
       51: bb4.i.i: ; preds = %bb3.i.i.i.i.1, %start
       52:  %5 = call i8* @__rust_alloc(i64 7404, i64 2) #5, !noalias !26
not:16                     !~~~~~~~~~~~~                                  error: no match expected
       53:  %6 = icmp eq i8* %5, null
       54:  br i1 %6, label %bb20.i.i.i.i.i, label %bb6.i.i.i
       55: 
       56: bb20.i.i.i.i.i: ; preds = %bb4.i.i
       57: ; call alloc::alloc::handle_alloc_error
        .
        .
>>>>>>
------------------------------------------
