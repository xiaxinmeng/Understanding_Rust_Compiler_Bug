plain
failures:

---- [codegen] codegen/vec-clear.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-clear/vec-clear.ll" "/checkout/src/test/codegen/vec-clear.rs" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/vec-clear.rs:8:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: load
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-clear/vec-clear.ll:17:18: note: found here
 %_2.i.i.i.i.i = load i32*, i32** %0, align 8, !alias.scope !12, !nonnull !17, !noundef !17
                 ^~~~
/checkout/src/test/codegen/vec-clear.rs:9:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: icmp
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-clear/vec-clear.ll:20:24: note: found here
 %_5.i.i.i.i.i.i.i.i = icmp eq i64 %_6.i.i.i.i.i.i.i.i, 0

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-clear/vec-clear.ll
Check file: /checkout/src/test/codegen/vec-clear.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
       .
       .
       .
       .
      12: start:
      13:  tail call void @llvm.experimental.noalias.scope.decl(metadata !2)
      14:  tail call void @llvm.experimental.noalias.scope.decl(metadata !5) #5, !noalias !2
      15:  tail call void @llvm.experimental.noalias.scope.decl(metadata !8) #5, !noalias !11
      16:  %0 = getelementptr inbounds %"alloc::vec::Vec<u32>", %"alloc::vec::Vec<u32>"* %x, i64 0, i32 0, i32 0
      17:  %_2.i.i.i.i.i = load i32*, i32** %0, align 8, !alias.scope !12, !nonnull !17, !noundef !17
not:8                      !~~~                                                                       error: no match expected
      18:  %1 = ptrtoint i32* %_2.i.i.i.i.i to i64
      19:  %_6.i.i.i.i.i.i.i.i = and i64 %1, 3
      20:  %_5.i.i.i.i.i.i.i.i = icmp eq i64 %_6.i.i.i.i.i.i.i.i, 0
not:9                            !~~~                               error: no match expected
      21:  br i1 %_5.i.i.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i.i, label %bb7.i.i.i.i.i.i.i
      22: 
      23: bb2.i.i.i.i.i.i.i: ; preds = %start
      24:  %2 = getelementptr inbounds %"alloc::vec::Vec<u32>", %"alloc::vec::Vec<u32>"* %x, i64 0, i32 1
      25:  %_7.i.i.i = load i64, i64* %2, align 8, !alias.scope !18
       .
       .
>>>>>>
------------------------------------------
