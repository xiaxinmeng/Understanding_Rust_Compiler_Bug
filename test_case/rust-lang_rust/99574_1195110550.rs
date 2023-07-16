plain
failures:

---- [codegen] src/test/codegen/box-maybe-uninit-llvm14.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/box-maybe-uninit-llvm14/box-maybe-uninit-llvm14.ll" "/checkout/src/test/codegen/box-maybe-uninit-llvm14.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/box-maybe-uninit-llvm14.rs:26:11: error: CHECK: expected string not found in input
// CHECK: declare noalias{{.*}} @__rust_alloc(i64, i64{{.*}})
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/box-maybe-uninit-llvm14/box-maybe-uninit-llvm14.ll:7:63: note: scanning from here
define noalias noundef nonnull align 4 i32* @box_uninitialized() unnamed_addr #0 {
                                                              ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/box-maybe-uninit-llvm14/box-maybe-uninit-llvm14.ll:28:1: note: possible intended match here
declare noalias i8* @__rust_alloc(i32, i32) unnamed_addr #2

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/box-maybe-uninit-llvm14/box-maybe-uninit-llvm14.ll
Check file: /checkout/src/test/codegen/box-maybe-uninit-llvm14.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'box_maybe_uninit_llvm14.883ee810-cgu.0' 
            2: source_filename = "box_maybe_uninit_llvm14.883ee810-cgu.0" 
            3: target datalayout = "e-m:e-p:32:32-p270:32:32-p271:32:32-p272:64:64-f64:32:64-f80:32-n8:16:32-S128" 
            4: target triple = "i586-unknown-linux-gnu" 
            5:  
            6: ; Function Attrs: nonlazybind uwtable 
            7: define noalias noundef nonnull align 4 i32* @box_uninitialized() unnamed_addr #0 { 
check:26'0                                                                   X~~~~~~~~~~~~~~~~~~~~ error: no match found
            8: start: 
check:26'0     ~~~~~~~
            9:  %0 = tail call align 4 dereferenceable_or_null(4) i8* @__rust_alloc(i32 4, i32 4) #3 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           10:  %1 = icmp eq i8* %0, null 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~
           11:  br i1 %1, label %bb1.i, label %_ZN5alloc5alloc15exchange_malloc17h02f06f4b268f41d0E.exit 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           12:  
check:26'0     ~
            .
            .
            .
           23: ; alloc::alloc::handle_alloc_error 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           24: ; Function Attrs: cold noreturn nonlazybind uwtable 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           25: declare void @_ZN5alloc5alloc18handle_alloc_error17haefdf631ca072821E(i32, i32 noundef) unnamed_addr #1 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           26:  
check:26'0     ~
           27: ; Function Attrs: nofree nounwind nonlazybind uwtable 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           28: declare noalias i8* @__rust_alloc(i32, i32) unnamed_addr #2 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:26'1     ?                                                            possible intended match
           29:  
check:26'0     ~
           30: attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="pentium" } 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           31: attributes #1 = { cold noreturn nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="pentium" } 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           32: attributes #2 = { nofree nounwind nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="pentium" } 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           33: attributes #3 = { nounwind } 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
