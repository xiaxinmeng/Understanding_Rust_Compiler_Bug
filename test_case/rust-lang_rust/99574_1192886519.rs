plain
 finished in 3.862 seconds
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 342 tests
ii...i.........i....i..i.................iii........ii.i..F....i.................ii..... 88/342
..................i...i...i.....ii..i.ii.............iiii........................i.i.ii. 264/342
i......i.......iii..........i.....................iiiiiii..i..................
failures:
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [codegen] src/test/codegen/box-maybe-uninit.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/box-maybe-uninit/box-maybe-uninit.ll" "/checkout/src/test/codegen/box-maybe-uninit.rs" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/box-maybe-uninit.rs:22:11: error: CHECK: expected string not found in input
// CHECK: declare noalias ptr @__rust_alloc(i64, i64 allocalign) unnamed_addr [[RUST_ALLOC_ATTRS:#[0-9]+]]
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/box-maybe-uninit/box-maybe-uninit.ll:10:63: note: scanning from here
define noalias noundef nonnull align 8 i64* @box_uninitialized() unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
                                                              ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/box-maybe-uninit/box-maybe-uninit.ll:34:1: note: possible intended match here
declare noalias i8* @__rust_alloc(i64, i64) unnamed_addr #2

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/box-maybe-uninit/box-maybe-uninit.ll
Check file: /checkout/src/test/codegen/box-maybe-uninit.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
            5: 
            6: %"unwind::libunwind::_Unwind_Exception" = type { i64, void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [6 x i64] }
            7: %"unwind::libunwind::_Unwind_Context" = type { [0 x i8] }
            8: 
            9: ; Function Attrs: nonlazybind uwtable
           10: define noalias noundef nonnull align 8 i64* @box_uninitialized() unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
check:22'0                                                                   X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           11: start:
check:22'0     ~~~~~~
           12:  %0 = tail call i8* @__rust_alloc(i64 8, i64 8) #3
check:22'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           13:  %1 = icmp eq i8* %0, null
check:22'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
           14:  br i1 %1, label %bb1.i, label %_ZN5alloc5alloc15exchange_malloc17h63d52770d270cdceE.exit
check:22'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           15: 
check:22'0     ~
            .
            .
            .
           29: ; alloc::alloc::handle_alloc_error
check:22'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           30: ; Function Attrs: cold noreturn nonlazybind uwtable
check:22'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           31: declare void @_ZN5alloc5alloc18handle_alloc_error17h7e89b09e250518d9E(i64, i64 noundef) unnamed_addr #1
check:22'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           32: 
check:22'0     ~
           33: ; Function Attrs: nounwind nonlazybind uwtable
check:22'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           34: declare noalias i8* @__rust_alloc(i64, i64) unnamed_addr #2
check:22'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:22'1     ?                                                           possible intended match
           35: 
check:22'0     ~
           36: attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
check:22'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           37: attributes #1 = { cold noreturn nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
check:22'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           38: attributes #2 = { nounwind nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
check:22'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           39: attributes #3 = { nounwind }
check:22'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
