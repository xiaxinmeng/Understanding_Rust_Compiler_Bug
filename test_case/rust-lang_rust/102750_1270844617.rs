plain
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 375 tests
i........i...........i.....i.ii.................iii........ii.i........i................ 88/375
.ii................i............i..i.................i...iii........i..i......iii.ii.F.. 176/375
.........i.i.ii.i.i............i...i...i......i.ii......i...i.....................iiiiii 352/375
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
ii.i...................
failures:
failures:

---- [codegen] src/test/codegen/issue-37945.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-37945/issue-37945.ll" "/checkout/src/test/codegen/issue-37945.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/issue-37945.rs:18:16: error: CHECK-NEXT: expected string not found in input
// CHECK-NEXT: [[A:%.*]] = icmp ne {{i32\*|ptr}} %xs.1, null
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-37945/issue-37945.ll:8:7: note: scanning from here
      ^
      ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-37945/issue-37945.ll:9:2: note: possible intended match here
 %0 = icmp ne i32* %xs.0, null
 ^
/checkout/src/test/codegen/issue-37945.rs:31:16: error: CHECK-NEXT: expected string not found in input
// CHECK-NEXT: [[C:%.*]] = icmp ne {{i32\*|ptr}} %xs.1, null
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-37945/issue-37945.ll:17:7: note: scanning from here
      ^
      ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-37945/issue-37945.ll:18:2: note: possible intended match here
 %0 = icmp ne i32* %xs.0, null

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-37945/issue-37945.ll
Check file: /checkout/src/test/codegen/issue-37945.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
           1: ; ModuleID = 'issue_37945.617b82f2-cgu.0' 
           2: source_filename = "issue_37945.617b82f2-cgu.0" 
           3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
           4: target triple = "x86_64-unknown-linux-gnu" 
           5:  
           6: ; Function Attrs: mustprogress nofree nosync nounwind nonlazybind uwtable willreturn 
           7: define noundef zeroext i1 @is_empty_1(i32* %xs.0, i32* noundef nonnull %xs.1) unnamed_addr #0 { 
           8: start: 
next:18'0           X error: no match found
           9:  %0 = icmp ne i32* %xs.0, null 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
next:18'1      ?                              possible intended match
          10:  tail call void @llvm.assume(i1 %0) 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          11:  %_10.i = icmp eq i32* %xs.0, %xs.1 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          12:  ret i1 %_10.i 
next:18'0     ~~~~~~~~~~~~~~~
          13: } 
next:18'0     ~~
          14:  
next:18'0     ~
          15: ; Function Attrs: mustprogress nofree nosync nounwind nonlazybind uwtable willreturn 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          16: define noundef zeroext i1 @is_empty_2(i32* %xs.0, i32* noundef nonnull %xs.1) unnamed_addr #0 { 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          17: start: 
next:31'0           X error: no match found
          18:  %0 = icmp ne i32* %xs.0, null 
next:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
next:31'1      ?                              possible intended match
          19:  tail call void @llvm.assume(i1 %0) 
next:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          20:  %_10.i = icmp eq i32* %xs.0, %xs.1 
next:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          21:  ret i1 %_10.i 
next:31'0     ~~~~~~~~~~~~~~~
          22: } 
next:31'0     ~~
          23:  
next:31'0     ~
           .
           .
>>>>>>
------------------------------------------
