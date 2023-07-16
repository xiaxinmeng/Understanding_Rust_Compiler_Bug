plain

running 377 tests
i.....i..............i....i..ii.................i..ii......ii.i........i................ 88/377
..ii.................i............i..i.................i...iii........i..i......i.iiii.. 176/377
......i.iii....i..................i..Fi...i.....ii..i.ii..................ii............ 264/377
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
iiii.i...................
failures:


---- [codegen] src/test/codegen/noalias-box-off.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/noalias-box-off/noalias-box-off.ll" "/checkout/src/test/codegen/noalias-box-off.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/noalias-box-off.rs:6:15: error: CHECK-NOT: excluded string found in input
// CHECK-NOT: noalias
              ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/noalias-box-off/noalias-box-off.ll:12:68: note: found here
 tail call void @__rust_dealloc(i8* nonnull %0, i64 1, i64 1) #2, !noalias !2


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/noalias-box-off/noalias-box-off.ll


-dump-input=help explains the following input dump.
Input was:
<<<<<<
       .
       .
       .
       .
       7: %"unwind::libunwind::_Unwind_Context" = type { [0 x i8] } 
       8:  
       9: ; Function Attrs: nounwind nonlazybind uwtable 
      10: define void @box_should_not_have_noalias_if_disabled(i8* noundef nonnull align 1 %0) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality { 
      11: start: 
      12:  tail call void @__rust_dealloc(i8* nonnull %0, i64 1, i64 1) #2, !noalias !2 
not:6                                                                        !~~~~~~     error: no match expected
      13:  ret void 
      14: } 
      15:  
      16: ; Function Attrs: nonlazybind uwtable 
      17: declare noundef i32 @rust_eh_personality(i32, i32 noundef, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*) unnamed_addr #1 
       .
       .
>>>>>>
------------------------------------------
