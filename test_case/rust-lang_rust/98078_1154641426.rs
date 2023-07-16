plain
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
...i.......iii..........i.....................iiiiiii...................
failures:

---- [codegen] src/test/codegen/issue-96497-slice-size-nowrap.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-96497-slice-size-nowrap/issue-96497-slice-size-nowrap.ll" "/checkout/src/test/codegen/issue-96497-slice-size-nowrap.rs" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/issue-96497-slice-size-nowrap.rs:26:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: store i32 42
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-96497-slice-size-nowrap/issue-96497-slice-size-nowrap.ll:54:2: note: found here
 store i32 42, i32* %5, align 4


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-96497-slice-size-nowrap/issue-96497-slice-size-nowrap.ll
Check file: /checkout/src/test/codegen/issue-96497-slice-size-nowrap.rs

-dump-input=help explains the following input dump.
Input was:
<<<<<<
        .
        .
        .
        .
       49:  br i1 %_4, label %"_ZN4core3ptr59drop_in_place$LT$alloc..boxed..Box$LT$$u5b$u32$u5d$$GT$$GT$17h0b3efb50cff86253E.exit", label %panic, !prof !3
       50: 
       51: "_ZN4core3ptr59drop_in_place$LT$alloc..boxed..Box$LT$$u5b$u32$u5d$$GT$$GT$17h0b3efb50cff86253E.exit": ; preds = %start
       52:  %4 = bitcast [0 x i32]* %0 to i8*
       53:  %5 = getelementptr inbounds [0 x i32], [0 x i32]* %0, i64 0, i64 1
       54:  store i32 42, i32* %5, align 4
not:26      !~~~~~~~~~~~                   error: no match expected
       55:  %6 = shl nsw i64 %1, 2
       56:  tail call void @__rust_dealloc(i8* nonnull %4, i64 %6, i64 4) #4
       57:  ret void
       58: 
       59: panic: ; preds = %start
        .
        .
>>>>>>
------------------------------------------
