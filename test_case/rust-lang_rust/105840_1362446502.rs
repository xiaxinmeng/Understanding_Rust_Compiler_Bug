plain
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 394 tests
i.....i..............i....i..ii.................iii........ii.i........i................ 88/394
...ii.................i............i..i..............i...F.i..iii........i..i.....i.iii. 176/394
ii........................i.i.ii.i.i............i..i....i......iiii.......i...ii........ 352/394
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
.............iiiiiii.ii...................
failures:
failures:

---- [codegen] src/test/codegen/integer-cmp.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/integer-cmp/integer-cmp.ll" "/checkout/src/test/codegen/integer-cmp.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/integer-cmp.rs:14:11: error: CHECK: expected string not found in input
// CHECK: icmp ne
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/integer-cmp/integer-cmp.ll:9:18: note: scanning from here
 %_5.i = icmp slt i64 %0, %1
                 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/integer-cmp/integer-cmp.ll:11:11: note: possible intended match here
 %_10.i = icmp sgt i64 %0, %1
/checkout/src/test/codegen/integer-cmp.rs:24:11: error: CHECK: expected string not found in input
/checkout/src/test/codegen/integer-cmp.rs:24:11: error: CHECK: expected string not found in input
// CHECK: icmp ne
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/integer-cmp/integer-cmp.ll:20:18: note: scanning from here
 %_5.i = icmp ult i32 %0, %1
                 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/integer-cmp/integer-cmp.ll:22:11: note: possible intended match here
 %_10.i = icmp ugt i32 %0, %1

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/integer-cmp/integer-cmp.ll
Check file: /checkout/src/test/codegen/integer-cmp.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'integer_cmp.5dc4e255-cgu.0' 
            2: source_filename = "integer_cmp.5dc4e255-cgu.0" 
            3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
            4: target triple = "x86_64-unknown-linux-gnu" 
            5:  
            6: ; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind readnone uwtable willreturn 
            7: define noundef i8 @cmp_signed(i64 %0, i64 %1) unnamed_addr #0 { 
            8: start: 
            9:  %_5.i = icmp slt i64 %0, %1 
check:14'0                      X~~~~~~~~~~~ error: no match found
           10:  %_4.neg.i = sext i1 %_5.i to i8 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           11:  %_10.i = icmp sgt i64 %0, %1 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:14'1               ?                    possible intended match
           12:  %_9.i = zext i1 %_10.i to i8 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           13:  %_13.0.i = add nsw i8 %_4.neg.i, %_9.i 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           14:  ret i8 %_13.0.i 
check:14'0     ~~~~~~~~~~~~~~~~~
           15: } 
check:14'0     ~~
           16:  
check:14'0     ~
           17: ; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind readnone uwtable willreturn 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           18: define noundef i8 @cmp_unsigned(i32 %0, i32 %1) unnamed_addr #0 { 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           19: start: 
           20:  %_5.i = icmp ult i32 %0, %1 
check:24'0                      X~~~~~~~~~~~ error: no match found
           21:  %_4.neg.i = sext i1 %_5.i to i8 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           22:  %_10.i = icmp ugt i32 %0, %1 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:24'1               ?                    possible intended match
           23:  %_9.i = zext i1 %_10.i to i8 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           24:  %_13.0.i = add nsw i8 %_4.neg.i, %_9.i 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           25:  ret i8 %_13.0.i 
check:24'0     ~~~~~~~~~~~~~~~~~
           26: } 
check:24'0     ~~
           27:  
check:24'0     ~
            .
            .
>>>>>>
------------------------------------------
