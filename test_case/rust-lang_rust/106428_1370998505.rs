
running 396 tests
iiiiii.........i....................i...............................i................... 88/396
i...........i...i...................i...........................ii...................... 176/396
i..ii...i.......ii...i.ii.i................ii........................................... 264/396
.....i....i....i..............................................i....i.................... 352/396
................F...........................
failures:

---- [codegen] src/test/codegen/unchecked_shifts.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/home/ben/rust/build/x86_64-unknown-linux-gnu/ci-llvm/bin/FileCheck" "--input-file" "/home/ben/rust/build/x86_64-unknown-linux-gnu/test/codegen/unchecked_shifts/unchecked_shifts.ll" "/home/ben/rust/src/test/codegen/unchecked_shifts.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/home/ben/rust/src/test/codegen/unchecked_shifts.rs:23:16: error: CHECK-DAG: expected string not found in input
 // CHECK-DAG: %[[INRANGE:.+]] = icmp ult i32 %b, 65536
               ^
/home/ben/rust/build/x86_64-unknown-linux-gnu/test/codegen/unchecked_shifts/unchecked_shifts.ll:14:43: note: scanning from here
define i16 @unchecked_shl_unsigned_smaller(i16 %a, i32 %b) unnamed_addr #0 {
                                          ^
/home/ben/rust/build/x86_64-unknown-linux-gnu/test/codegen/unchecked_shifts/unchecked_shifts.ll:16:5: note: possible intended match here
 %_2.i.i = icmp ugt i32 %b, 65535
    ^
/home/ben/rust/src/test/codegen/unchecked_shifts.rs:53:16: error: CHECK-DAG: expected string not found in input
 // CHECK-DAG: %[[INRANGE:.+]] = icmp ult i32 %b, 32768
               ^
/home/ben/rust/build/x86_64-unknown-linux-gnu/test/codegen/unchecked_shifts/unchecked_shifts.ll:39:41: note: scanning from here
define i16 @unchecked_shr_signed_smaller(i16 %a, i32 %b) unnamed_addr #0 {
                                        ^
/home/ben/rust/build/x86_64-unknown-linux-gnu/test/codegen/unchecked_shifts/unchecked_shifts.ll:41:5: note: possible intended match here
 %_2.i.i = icmp ugt i32 %b, 32767
    ^

Input file: /home/ben/rust/build/x86_64-unknown-linux-gnu/test/codegen/unchecked_shifts/unchecked_shifts.ll
Check file: /home/ben/rust/src/test/codegen/unchecked_shifts.rs

-dump-input=help explains the following input dump.

Input was:
<<<<<<
          1: ; ModuleID = 'unchecked_shifts.d7deabc1-cgu.0' 
          2: source_filename = "unchecked_shifts.d7deabc1-cgu.0" 
          3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
          4: target triple = "x86_64-unknown-linux-gnu" 
          5:  
          6: ; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind readnone willreturn uwtable 
          7: define i32 @unchecked_shl_unsigned_same(i32 %a, i32 %b) unnamed_addr #0 { 
          8: start: 
          9:  %0 = shl i32 %a, %b 
         10:  ret i32 %0 
         11: } 
         12:  
         13: ; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind readnone willreturn uwtable 
         14: define i16 @unchecked_shl_unsigned_smaller(i16 %a, i32 %b) unnamed_addr #0 { 
dag:23'0                                               X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
         15: start: 
dag:23'0     ~~~~~~~
         16:  %_2.i.i = icmp ugt i32 %b, 65535 
dag:23'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
dag:23'1         ?                              possible intended match
         17:  %_5.i.i = trunc i32 %b to i16 
dag:23'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         18:  %.sroa.3.0.i.i = select i1 %_2.i.i, i16 undef, i16 %_5.i.i 
dag:23'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         19:  %0 = shl i16 %a, %.sroa.3.0.i.i 
dag:23'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         20:  ret i16 %0 
dag:23'0     ~~~~~~~~~~~~
         21: } 
dag:23'0     ~~
         22:  
dag:23'0     ~
         23: ; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind readnone willreturn uwtable 
dag:23'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         24: define i64 @unchecked_shl_unsigned_bigger(i64 %a, i32 %b) unnamed_addr #0 { 
dag:23'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         25: start: 
         26:  %0 = zext i32 %b to i64 
         27:  %1 = shl i64 %a, %0 
         28:  ret i64 %1 
         29: } 
         30:  
         31: ; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind readnone willreturn uwtable 
         32: define i32 @unchecked_shr_signed_same(i32 %a, i32 %b) unnamed_addr #0 { 
         33: start: 
         34:  %0 = ashr i32 %a, %b 
         35:  ret i32 %0 
         36: } 
         37:  
         38: ; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind readnone willreturn uwtable 
         39: define i16 @unchecked_shr_signed_smaller(i16 %a, i32 %b) unnamed_addr #0 { 
dag:53'0                                             X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
         40: start: 
dag:53'0     ~~~~~~~
         41:  %_2.i.i = icmp ugt i32 %b, 32767 
dag:53'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
dag:53'1         ?                              possible intended match
         42:  %_5.i.i = trunc i32 %b to i16 
dag:53'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         43:  %.sroa.3.0.i.i = select i1 %_2.i.i, i16 undef, i16 %_5.i.i 
dag:53'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         44:  %0 = ashr i16 %a, %.sroa.3.0.i.i 
dag:53'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         45:  ret i16 %0 
dag:53'0     ~~~~~~~~~~~~
         46: } 
dag:53'0     ~~
         47:  
dag:53'0     ~
         48: ; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind readnone willreturn uwtable 
dag:53'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         49: define i64 @unchecked_shr_signed_bigger(i64 %a, i32 %b) unnamed_addr #0 { 
dag:53'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         50: start: 
         51:  %0 = zext i32 %b to i64 
         52:  %1 = ashr i64 %a, %0 
         53:  ret i64 %1 
         54: } 
         55:  
         56: attributes #0 = { mustprogress nofree norecurse nosync nounwind nonlazybind readnone willreturn uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
         57:  
         58: !llvm.module.flags = !{!0, !1} 
         59:  
         60: !0 = !{i32 7, !"PIC Level", i32 2} 
         61: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
>>>>>>
------------------------------------------



failures:
    [codegen] src/test/codegen/unchecked_shifts.rs

test result: FAILED. 363 passed; 1 failed; 32 ignored; 0 measured; 0 filtered out; finished in 2.37s

Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
