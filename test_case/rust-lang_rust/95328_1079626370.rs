plain
failures:

---- [codegen] codegen/loads.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/loads/loads.ll" "/checkout/src/test/codegen/loads.rs" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/loads.rs:125:11: error: CHECK: expected string not found in input
// CHECK: load {{(i32\*, )?}}i32** %x{{.*}}, !nonnull
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/loads/loads.ll:612:17: note: scanning from here
define i32 @_box(i32* noalias noundef nonnull align 4 %0) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
                ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/loads/loads.ll:617:7: note: possible intended match here
 %2 = load i32*, i32** %x, align 8

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/loads/loads.ll
Check file: /checkout/src/test/codegen/loads.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
             .
             .
             .
             .
           607:  %1 = load i32*, i32** %x, align 8, !nonnull !3, !align !4, !noundef !3
           608:  ret i32* %1
           609: }
           610: 
           611: ; Function Attrs: nonlazybind uwtable
           612: define i32 @_box(i32* noalias noundef nonnull align 4 %0) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
check:125'0                     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           613: start:
check:125'0     ~~~~~~
           614:  %1 = alloca { i8*, i32 }, align 8
check:125'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           615:  %x = alloca i32*, align 8
check:125'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
           616:  store i32* %0, i32** %x, align 8
check:125'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           617:  %2 = load i32*, i32** %x, align 8
check:125'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:125'1           ?                            possible intended match
           618:  %3 = load i32, i32* %2, align 4
check:125'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           619: ; invoke core::ptr::drop_in_place<alloc::boxed::Box<i32>>
check:125'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           620:  invoke void @"_ZN4core3ptr49drop_in_place$LT$alloc..boxed..Box$LT$i32$GT$$GT$17h78d62f8a4e32a77eE"(i32** %x)
check:125'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           621:  to label %bb1 unwind label %cleanup
check:125'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           622: 
check:125'0     ~
             .
             .
>>>>>>
------------------------------------------
