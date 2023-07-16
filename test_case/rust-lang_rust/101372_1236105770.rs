plain
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

failures:

---- [codegen] src/test/codegen/issue-98294-get-mut-copy-from-slice-opt.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-98294-get-mut-copy-from-slice-opt/issue-98294-get-mut-copy-from-slice-opt.ll" "/checkout/src/test/codegen/issue-98294-get-mut-copy-from-slice-opt.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/issue-98294-get-mut-copy-from-slice-opt.rs:11:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: call
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-98294-get-mut-copy-from-slice-opt/issue-98294-get-mut-copy-from-slice-opt.ll:29:7: note: found here
 tail call void @llvm.trap(), !noalias !2


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-98294-get-mut-copy-from-slice-opt/issue-98294-get-mut-copy-from-slice-opt.ll
Check file: /checkout/src/test/codegen/issue-98294-get-mut-copy-from-slice-opt.rs

-dump-input=help explains the following input dump.
Input was:
<<<<<<
        .
        .
        .
        .
       24:  %diff.0.i.i.i.i.i = select i1 %_13.i.i.i.i.i, i64 %_18.0.i.i.i.i.i, i64 %_21.0.i.i.i.i.i 
       25:  %.not.i.i.i.i = icmp ult i64 %diff.0.i.i.i.i.i, %.sroa.3.0.i 
       26:  br i1 %.not.i.i.i.i, label %bb10.i.i.i.i, label %"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$15copy_from_slice17hdf5e891f94371aebE.exit" 
       27:  
       28: bb10.i.i.i.i: ; preds = %bb2.i.i.i.i 
       29:  tail call void @llvm.trap(), !noalias !2 
not:11           !~~~                                 error: no match expected
       30:  unreachable 
       31:  
       32: "_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$15copy_from_slice17hdf5e891f94371aebE.exit": ; preds = %bb2.i.i.i.i 
       33:  tail call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %.sroa.0.0.i, ptr nonnull align 1 %bytes.0, i64 %.sroa.3.0.i, i1 false), !alias.scope !9 
       34:  br label %bb3 
        .
        .
>>>>>>
------------------------------------------
