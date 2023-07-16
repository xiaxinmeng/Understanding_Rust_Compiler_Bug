plain
failures:

---- [codegen] checkout/tests/codegen/vec-as-ptr.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-as-ptr/vec-as-ptr.ll" "/checkout/tests/codegen/vec-as-ptr.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/tests/codegen/vec-as-ptr.rs:5:11: error: CHECK: expected string not found in input
// CHECK: nonnull ptr @vec_as_ptr(ptr noalias nocapture noundef readonly align {{.*}} dereferenceable({{.*}} %v)
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-as-ptr/vec-as-ptr.ll:1:1: note: scanning from here
; ModuleID = 'vec_as_ptr.7f58bd3c-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-as-ptr/vec-as-ptr.ll:9:21: note: possible intended match here
define nonnull i8* @vec_as_ptr(%"alloc::vec::Vec<u8>"* noalias nocapture noundef readonly align 8 dereferenceable(24) %v) unnamed_addr #0 {

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-as-ptr/vec-as-ptr.ll
Check file: /checkout/tests/codegen/vec-as-ptr.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
           1: ; ModuleID = 'vec_as_ptr.7f58bd3c-cgu.0' 
check:5'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           2: source_filename = "vec_as_ptr.7f58bd3c-cgu.0" 
check:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
check:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           4: target triple = "x86_64-unknown-linux-gnu" 
check:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           5:  
check:5'0     ~
           6: %"alloc::vec::Vec<u8>" = type { { i64, i8* }, i64 } 
check:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           7:  
check:5'0     ~
           8: ; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind readonly uwtable willreturn 
check:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           9: define nonnull i8* @vec_as_ptr(%"alloc::vec::Vec<u8>"* noalias nocapture noundef readonly align 8 dereferenceable(24) %v) unnamed_addr #0 { 
check:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:5'1                         ?                                                                                                                        possible intended match
          10: start: 
check:5'0     ~~~~~~~
          11:  %0 = getelementptr inbounds %"alloc::vec::Vec<u8>", %"alloc::vec::Vec<u8>"* %v, i64 0, i32 0, i32 1 
check:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          12:  %self1 = load i8*, i8** %0, align 8, !nonnull !2, !noundef !2 
check:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          13:  ret i8* %self1 
check:5'0     ~~~~~~~~~~~~~~~~
          14: } 
check:5'0     ~~
          15:  
check:5'0     ~
          16: attributes #0 = { mustprogress nofree norecurse nosync nounwind nonlazybind readonly uwtable willreturn "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
check:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          17:  
check:5'0     ~
          18: !llvm.module.flags = !{!0, !1} 
check:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          19:  
check:5'0     ~
          20: !0 = !{i32 7, !"PIC Level", i32 2} 
check:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          21: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
check:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          22: !2 = !{} 
check:5'0     ~~~~~~~~~
------------------------------------------



