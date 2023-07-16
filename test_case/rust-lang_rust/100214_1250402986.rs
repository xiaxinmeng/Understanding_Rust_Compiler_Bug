plain
failures:

---- [codegen] src/test/codegen/slice-iter-len-eq-zero.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-iter-len-eq-zero/slice-iter-len-eq-zero.ll" "/checkout/src/test/codegen/slice-iter-len-eq-zero.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/slice-iter-len-eq-zero.rs:22:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: icmp
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-iter-len-eq-zero/slice-iter-len-eq-zero.ll:18:17: note: found here
 %_18.i.i.i.i = icmp ugt i64 %_3.i, 123


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-iter-len-eq-zero/slice-iter-len-eq-zero.ll


-dump-input=help explains the following input dump.
Input was:
<<<<<<
        .
        .
        .
        .
       13: ; Function Attrs: nonlazybind uwtable 
       14: define noundef zeroext i1 @array_into_iter_len_eq_zero(ptr noalias nocapture noundef readonly dereferenceable(392) %y) unnamed_addr #1 personality ptr @rust_eh_personality { 
       15: bb1: 
       16:  %0 = getelementptr inbounds { i64, i64 }, ptr %y, i64 0, i32 1 
       17:  %_3.i = load i64, ptr %0, align 8, !alias.scope !2 
       18:  %_18.i.i.i.i = icmp ugt i64 %_3.i, 123 
not:22                     !~~~                     error: no match expected
       19:  br i1 %_18.i.i.i.i, label %bb1.i.i.i.i, label %"_ZN4core3ptr91drop_in_place$LT$core..array..iter..IntoIter$LT$$u5b$u8$u3b$$u20$3$u5d$$C$123_usize$GT$$GT$17hbdfa8dea7c1f446aE.exit" 
       20:  
       21: bb1.i.i.i.i: ; preds = %bb1 
       22:  tail call void @llvm.trap(), !noalias !5 
       23:  unreachable 
        .
        .
>>>>>>
------------------------------------------
