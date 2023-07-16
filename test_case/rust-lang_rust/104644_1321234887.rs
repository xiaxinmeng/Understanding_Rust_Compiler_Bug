plain
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 383 tests
i.....i..............i....i..ii.................iii........ii.i........i................ 88/383
..iiF................i............i..i..................i...iii......F.i..i.....i.iii.ii 176/383
i.........i.iii.....i..................i...i...i.....ii..i.ii....F............ii........ 264/383
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
..iiiiiiii.i...................
failures:


---- [codegen] src/test/codegen/drop.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/drop/drop.ll" "/checkout/src/test/codegen/drop.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/drop.rs:26:19: error: CHECK-COUNT: expected string not found in input (4 out of 6)
// CHECK-COUNT-6: {{(call|invoke) void @.*}}drop_in_place{{.*}}SomeUniqueName
                  ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/drop/drop.ll:92:108: note: scanning from here
 invoke void @"_ZN4core3ptr41drop_in_place$LT$drop..SomeUniqueName$GT$17h34dbcfbf8ec05bc7E"(%SomeUniqueName* %_s2) #3
                                                                                                           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/drop/drop.ll:110:2: note: possible intended match here
 invoke void @"_ZN62_$LT$drop..SomeUniqueName$u20$as$u20$core..ops..drop..Drop$GT$4drop17h037000ea3ece15acE"(%SomeUniqueName* noalias noundef nonnull align 1 %_s2)

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/drop/drop.ll
Check file: /checkout/src/test/codegen/drop.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
           87:  invoke void @_ZN4drop18possibly_unwinding17h61c28a7aede8c5dfE() 
           88:  to label %bb3 unwind label %cleanup4 
           89:  
           90: bb4: ; preds = %cleanup4 
           91: ; invoke core::ptr::drop_in_place<drop::SomeUniqueName> 
           92:  invoke void @"_ZN4core3ptr41drop_in_place$LT$drop..SomeUniqueName$GT$17h34dbcfbf8ec05bc7E"(%SomeUniqueName* %_s2) #3 
count:26'0                                                                                                                X~~~~~~~~~~ error: no match found
           93:  to label %bb5 unwind label %abort 
count:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           94:  
count:26'0     ~
           95: cleanup4: ; preds = %bb2 
count:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~
           96:  %13 = landingpad { i8*, i32 } 
count:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           97:  cleanup 
count:26'0     ~~~~~~~~~
            .
            .
            .
          105:  store i32 %16, i32* %18, align 8 
count:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          106:  br label %bb4 
count:26'0     ~~~~~~~~~~~~~~~
          107:  
count:26'0     ~
          108: bb3: ; preds = %bb2 
count:26'0     ~~~~~~~~~~~~~~~~~~~~
          109: ; invoke <drop::SomeUniqueName as core::ops::drop::Drop>::drop 
count:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          110:  invoke void @"_ZN62_$LT$drop..SomeUniqueName$u20$as$u20$core..ops..drop..Drop$GT$4drop17h037000ea3ece15acE"(%SomeUniqueName* noalias noundef nonnull align 1 %_s2) 
count:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
count:26'1      ?                                                                                                                                                                   possible intended match
          111:  to label %bb8 unwind label %cleanup3 
count:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          112:  
count:26'0     ~
          113: abort: ; preds = %bb6, %bb5, %bb4 
count:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          114:  %19 = landingpad { i8*, i32 } 
count:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          115:  cleanup 
count:26'0     ~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
------------------------------------------


---- [codegen] src/test/codegen/inline-hint.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/inline-hint/inline-hint.ll" "/checkout/src/test/codegen/inline-hint.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/inline-hint.rs:19:11: error: CHECK: expected string not found in input
// CHECK: ; core::ptr::drop_in_place::<inline_hint::A>
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/inline-hint/inline-hint.ll:1:1: note: scanning from here
; ModuleID = 'inline_hint.e9e66dcf-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/inline-hint/inline-hint.ll:31:1: note: possible intended match here
; core::ptr::drop_in_place::<alloc::vec::Vec<u8>>

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/inline-hint/inline-hint.ll
Check file: /checkout/src/test/codegen/inline-hint.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'inline_hint.e9e66dcf-cgu.0' 
check:19'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            2: source_filename = "inline_hint.e9e66dcf-cgu.0" 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            4: target triple = "x86_64-unknown-linux-gnu" 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            5:  
check:19'0     ~
            6: %"alloc::vec::Vec<u8>" = type { { i8*, i64 }, i64 } 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
            .
           26: @alloc105 = private unnamed_addr constant <{ [42 x i8] }> <{ [42 x i8] c"/checkout/library/core/src/alloc/layout.rs" }>, align 1 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           27: @alloc106 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [42 x i8] }>, <{ [42 x i8] }>* @alloc105, i32 0, i32 0, i32 0), [16 x i8] c"*\00\00\00\00\00\00\00\B6\01\00\00)\00\00\00" }>, align 8 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           28: @str.0 = internal constant [25 x i8] c"attempt to divide by zero" 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           29: @0 = private unnamed_addr constant <{ [16 x i8] }> <{ [16 x i8] c"\01\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00" }>, align 8 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           30:  
check:19'0     ~
           31: ; core::ptr::drop_in_place::<alloc::vec::Vec<u8>> 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:19'1     ?                                                  possible intended match
           32: ; Function Attrs: nonlazybind uwtable 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           33: define internal void @_RINvNtCsiXyDOIUVBBm_4core3ptr13drop_in_placeINtNtCsjJML2pz3l2X_5alloc3vec3VechEECsleXdIPmWM9L_11inline_hint(%"alloc::vec::Vec<u8>"* %_1) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality { 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           34: start: 
check:19'0     ~~~~~~~
           35:  %0 = alloca { i8*, i32 }, align 8 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           36: ; invoke <alloc::vec::Vec<u8> as core::ops::drop::Drop>::drop 
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
------------------------------------------


---- [codegen] src/test/codegen/panic-in-drop-abort.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/panic-in-drop-abort/panic-in-drop-abort.ll" "/checkout/src/test/codegen/panic-in-drop-abort.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/panic-in-drop-abort.rs:11:15: error: CHECK-NOT: excluded string found in input
// CHECK-NOT: {{(call|invoke).*}}should_not_appear_in_output
              ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/panic-in-drop-abort/panic-in-drop-abort.ll:15:2: note: found here
 invoke void @should_not_appear_in_output()

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/panic-in-drop-abort/panic-in-drop-abort.ll
Check file: /checkout/src/test/codegen/panic-in-drop-abort.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
        .
        .
        .
        .
       10:  
       11: ; core::ptr::drop_in_place<panic_in_drop_abort::AssertNeverDrop> 
       12: ; Function Attrs: nounwind nonlazybind uwtable 
       13: define internal fastcc void @"_ZN4core3ptr57drop_in_place$LT$panic_in_drop_abort..AssertNeverDrop$GT$17h28549854766a4e89E"() unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality { 
       14: start: 
       15:  invoke void @should_not_appear_in_output() 
not:11      !~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~    error: no match expected
       16:  to label %bb1 unwind label %cleanup 
       17:  
       18: cleanup: ; preds = %start 
       19:  %0 = landingpad { i8*, i32 } 
       20:  cleanup 
        .
        .
>>>>>>
------------------------------------------
