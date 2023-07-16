
---- [codegen] codegen/issue-75742-format_without_fmt_args.rs stdout ----

error: verification with 'FileCheck' failed
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-75742-format_without_fmt_args/issue-75742-format_without_fmt_args.ll" "/checkout/src/test/codegen/issue-75742-format_without_fmt_args.rs"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/checkout/src/test/codegen/issue-75742-format_without_fmt_args.rs:10:16: error: CHECK-NEXT: is not on the line after the previous match
// CHECK-NEXT: {{"_ZN[^:]+"}}:
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-75742-format_without_fmt_args/issue-75742-format_without_fmt_args.ll:195:1: note: 'next' match was here
"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$7dealloc17hcbba748b2e896a43E.exit.i.i.i.i25": ; preds = %bb8
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-75742-format_without_fmt_args/issue-75742-format_without_fmt_args.ll:176:32: note: previous match ended here
define void @format_wo_fmt_args() unnamed_addr #2 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
                               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-75742-format_without_fmt_args/issue-75742-format_without_fmt_args.ll:177:1: note: non-matching line after previous match is here
bb8:
^

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-75742-format_without_fmt_args/issue-75742-format_without_fmt_args.ll
Check file: /checkout/src/test/codegen/issue-75742-format_without_fmt_args.rs

-dump-input=help explains the following input dump.

Input was:
<<<<<<
         .
         .
         .
       190:  %_4.i.i.i.i.i22 = icmp eq i64 %_1.sroa.4.0.copyload, 0
       191:  %.not.i.i.i.i23 = icmp eq i8* %_1.sroa.0.0.copyload, null
       192:  %or.cond.i.i.i.i24 = or i1 %.not.i.i.i.i23, %_4.i.i.i.i.i22
       193:  br i1 %or.cond.i.i.i.i24, label %bb17, label %"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$7dealloc17hcbba748b2e896a43E.exit.i.i.i.i25"
       194: 
       195: "_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$7dealloc17hcbba748b2e896a43E.exit.i.i.i.i25": ; preds = %bb8
next:10     !~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~                error: match on wrong line
       196:  tail call void @__rust_dealloc(i8* nonnull %_1.sroa.0.0.copyload, i64 %_1.sroa.4.0.copyload, i64 1) #7
       197:  br label %bb17
       198: 
       199: bb17: ; preds = %"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$7dealloc17hcbba748b2e896a43E.exit.i.i.i.i25", %bb8
       200:  %1 = bitcast %"std::string::String"* %r1 to i8*
         .
         .
         .
>>>>>>

------------------------------------------



failures:
    [codegen] codegen/issue-75742-format_without_fmt_args.rs
