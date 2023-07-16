plain
failures:

---- [codegen] tests/codegen/unchecked_shifts.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/ci-llvm/bin/FileCheck" "--input-file" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/codegen/unchecked_shifts/unchecked_shifts.ll" "/Users/runner/work/rust/rust/tests/codegen/unchecked_shifts.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/Users/runner/work/rust/rust/tests/codegen/unchecked_shifts.rs:23:16: error: CHECK-DAG: expected string not found in input
 // CHECK-DAG: %[[INRANGE:.+]] = icmp ult i32 %b, 65536
               ^
/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/codegen/unchecked_shifts/unchecked_shifts.ll:14:51: note: scanning from here
define noundef i16 @unchecked_shl_unsigned_smaller(i16 noundef %a, i32 noundef %b) unnamed_addr #0 {
                                                  ^
/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/codegen/unchecked_shifts/unchecked_shifts.ll:16:2: note: possible intended match here
 %_3 = trunc i32 %b to i16
 ^
/Users/runner/work/rust/rust/tests/codegen/unchecked_shifts.rs:53:16: error: CHECK-DAG: expected string not found in input
 // CHECK-DAG: %[[INRANGE:.+]] = icmp ult i32 %b, 32768
               ^
/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/codegen/unchecked_shifts/unchecked_shifts.ll:37:49: note: scanning from here
define noundef i16 @unchecked_shr_signed_smaller(i16 noundef %a, i32 noundef %b) unnamed_addr #0 {
                                                ^
/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/codegen/unchecked_shifts/unchecked_shifts.ll:39:2: note: possible intended match here
 %_3 = trunc i32 %b to i16

Input file: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/codegen/unchecked_shifts/unchecked_shifts.ll
Check file: /Users/runner/work/rust/rust/tests/codegen/unchecked_shifts.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
          1: ; ModuleID = 'unchecked_shifts.d420e61f-cgu.0' 
          2: source_filename = "unchecked_shifts.d420e61f-cgu.0" 
          3: target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
          4: target triple = "x86_64-apple-macosx10.8.0" 
          5:  
          6: ; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable 
          7: define noundef i32 @unchecked_shl_unsigned_same(i32 noundef %a, i32 noundef %b) unnamed_addr #0 { 
          8: start: 
          9:  %0 = shl i32 %a, %b 
         10:  ret i32 %0 
         11: } 
         12:  
         13: ; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable 
         14: define noundef i16 @unchecked_shl_unsigned_smaller(i16 noundef %a, i32 noundef %b) unnamed_addr #0 { 
dag:23'0                                                       X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
         15: start: 
dag:23'0     ~~~~~~~
         16:  %_3 = trunc i32 %b to i16 
dag:23'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~
dag:23'1      ?                          possible intended match
         17:  %0 = shl i16 %a, %_3 
dag:23'0     ~~~~~~~~~~~~~~~~~~~~~~
         18:  ret i16 %0 
dag:23'0     ~~~~~~~~~~~~
         19: } 
dag:23'0     ~~
         20:  
dag:23'0     ~
         21: ; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable 
dag:23'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         22: define noundef i64 @unchecked_shl_unsigned_bigger(i64 noundef %a, i32 noundef %b) unnamed_addr #0 { 
dag:23'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         23: start: 
         24:  %_3 = zext i32 %b to i64 
         25:  %0 = shl i64 %a, %_3 
         26:  ret i64 %0 
         27: } 
         28:  
         29: ; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable 
         30: define noundef i32 @unchecked_shr_signed_same(i32 noundef %a, i32 noundef %b) unnamed_addr #0 { 
         31: start: 
         32:  %0 = ashr i32 %a, %b 
         33:  ret i32 %0 
         34: } 
         35:  
         36: ; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable 
         37: define noundef i16 @unchecked_shr_signed_smaller(i16 noundef %a, i32 noundef %b) unnamed_addr #0 { 
dag:53'0                                                     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
         38: start: 
dag:53'0     ~~~~~~~
         39:  %_3 = trunc i32 %b to i16 
dag:53'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~
dag:53'1      ?                          possible intended match
         40:  %0 = ashr i16 %a, %_3 
dag:53'0     ~~~~~~~~~~~~~~~~~~~~~~~
         41:  ret i16 %0 
dag:53'0     ~~~~~~~~~~~~
         42: } 
dag:53'0     ~~
         43:  
dag:53'0     ~
         44: ; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable 
dag:53'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         45: define noundef i64 @unchecked_shr_signed_bigger(i64 noundef %a, i32 noundef %b) unnamed_addr #0 { 
dag:53'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         46: start: 
         47:  %_3 = zext i32 %b to i64 
         48:  %0 = ashr i64 %a, %_3 
         49:  ret i64 %0 
         50: } 
         51:  
         52: attributes #0 = { mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable "frame-pointer"="all" "probe-stack"="inline-asm" "target-cpu"="core2" } 
         53:  
         54: !llvm.module.flags = !{!0} 
         55:  
         56: !0 = !{i32 8, !"PIC Level", i32 2} 
------------------------------------------



