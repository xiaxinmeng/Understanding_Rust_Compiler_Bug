plain
failures:

---- [codegen] codegen/loads.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/loads/loads.ll" "/checkout/src/test/codegen/loads.rs" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/loads.rs:61:11: error: CHECK: expected string not found in input
// CHECK: load i32*, i32** %x, align [[PTR_ALIGNMENT]], !nonnull !{{[0-9]+}}, !align ![[ALIGN_4_META]], !noundef !{{[0-9]+}}
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/loads/loads.ll:661:54: note: scanning from here
define noalias noundef nonnull align 4 i32* @load_box(i32** noalias noundef nonnull align 8 %x) unnamed_addr #1 {
                                                     ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/loads/loads.ll:661:54: note: with "PTR_ALIGNMENT" equal to "8"
define noalias noundef nonnull align 4 i32* @load_box(i32** noalias noundef nonnull align 8 %x) unnamed_addr #1 {
                                                     ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/loads/loads.ll:661:54: note: with "ALIGN_4_META" equal to "5"
define noalias noundef nonnull align 4 i32* @load_box(i32** noalias noundef nonnull align 8 %x) unnamed_addr #1 {
                                                     ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/loads/loads.ll:667:7: note: possible intended match here
 %4 = load i32*, i32** %3, align 8, !nonnull !2, !align !5, !noundef !2
      ^
/checkout/src/test/codegen/loads.rs:125:11: error: CHECK: expected string not found in input
// CHECK: load {{(i32\*, )?}}i32** %x{{.*}}, !nonnull
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/loads/loads.ll:738:17: note: scanning from here
define i32 @_box(i32* noalias noundef nonnull align 4 %0) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
                ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/loads/loads.ll:743:7: note: possible intended match here
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
           656:  %0 = load i32*, i32** %x, align 8
           657:  ret i32* %0
           658: }
           659: 
           660: ; Function Attrs: nonlazybind uwtable
           661: define noalias noundef nonnull align 4 i32* @load_box(i32** noalias noundef nonnull align 8 %x) unnamed_addr #1 {
check:61'0                                                           X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
check:61'1                                                                                                                        with "PTR_ALIGNMENT" equal to "8"
check:61'2                                                                                                                        with "ALIGN_4_META" equal to "5"
           662: start:
check:61'0      ~~~~~~
           663:  %0 = bitcast i32** %x to i64*
check:61'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           664:  %1 = bitcast i64* %0 to i32**
check:61'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           665:  %2 = bitcast i32** %x to i64*
check:61'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           666:  %3 = bitcast i64* %2 to i32**
check:61'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           667:  %4 = load i32*, i32** %3, align 8, !nonnull !2, !align !5, !noundef !2
check:61'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:61'3            ?                                                                 possible intended match
           668:  %5 = bitcast i32** %x to i64*
check:61'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           669: ; call alloc::alloc::box_free
check:61'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           670:  call void @_ZN5alloc5alloc8box_free17hd87cf3ef98b0b7ccE(i64* noundef nonnull %5)
check:61'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           671:  br label %bb1
check:61'0      ~~~~~~~~~~~~~~
           672: 
check:61'0      ~
             .
             .
             .
           733:  %1 = load i32*, i32** %x, align 8, !nonnull !2, !align !5, !noundef !2
           734:  ret i32* %1
           735: }
           736: 
           737: ; Function Attrs: nonlazybind uwtable
           738: define i32 @_box(i32* noalias noundef nonnull align 4 %0) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
check:125'0                     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           739: start:
check:125'0     ~~~~~~
           740:  %1 = alloca { i8*, i32 }, align 8
check:125'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           741:  %x = alloca i32*, align 8
check:125'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
           742:  store i32* %0, i32** %x, align 8
check:125'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           743:  %2 = load i32*, i32** %x, align 8
check:125'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:125'1           ?                            possible intended match
           744:  %3 = load i32, i32* %2, align 4
check:125'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           745: ; invoke core::ptr::drop_in_place<alloc::boxed::Box<i32>>
check:125'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           746:  invoke void @"_ZN4core3ptr49drop_in_place$LT$alloc..boxed..Box$LT$i32$GT$$GT$17h323a164ee2de1a7cE"(i32** %x)
check:125'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           747:  to label %bb1 unwind label %cleanup
check:125'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           748: 
check:125'0     ~
             .
             .
>>>>>>
------------------------------------------
