plain
Suite(test::src/test/codegen) not skipped for "bootstrap::test::Codegen" -- not in [src/tools/tidy]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 330 tests
ii....i.......Fi...i..i.................iii....F..iii.......i.................ii.................i.. 100/330
............i..........F.....i..iii........i..i..............i.......i..............i...i...i.....ii 200/330
..i.ii............Fiiii.........................iii.i......i.......iii..........i................... 300/330
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [codegen] codegen/abi-repr-ext.rs stdout ----


error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-repr-ext/abi-repr-ext.ll" "/checkout/src/test/codegen/abi-repr-ext.rs" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/abi-repr-ext.rs:11:11: error: CHECK: expected string not found in input
// CHECK: define{{( dso_local)?}} noundef signext i8 @test()
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-repr-ext/abi-repr-ext.ll:1:1: note: scanning from here
; ModuleID = 'abi_repr_ext.d2cf9aa6-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-repr-ext/abi-repr-ext.ll:6:43: note: possible intended match here
; Function Attrs: norecurse nounwind nonlazybind readnone uwtable willreturn


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-repr-ext/abi-repr-ext.ll


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'abi_repr_ext.d2cf9aa6-cgu.0'
check:11'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            2: source_filename = "abi_repr_ext.d2cf9aa6-cgu.0"
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            4: target triple = "x86_64-unknown-linux-gnu"
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            5: 
check:11'0     ~
            6: ; Function Attrs: norecurse nounwind nonlazybind readnone uwtable willreturn
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:11'1                                               ?                                  possible intended match
            7: define signext i8 @test() unnamed_addr #0 {
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            8: start:
check:11'0     ~~~~~~
            9:  ret i8 0
check:11'0     ~~~~~~~~~
           10: }
check:11'0     ~
           11: 
check:11'0     ~
            .
            .
>>>>>>
------------------------------------------
------------------------------------------


---- [codegen] codegen/call-metadata.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/call-metadata/call-metadata.ll" "/checkout/src/test/codegen/call-metadata.rs" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/call-metadata.rs:9:12: error: CHECK: expected string not found in input
 // CHECK: call noundef i8 @some_true(), !range [[R0:![0-9]+]]
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/call-metadata/call-metadata.ll:1:1: note: scanning from here
; ModuleID = 'call_metadata.cd7be62b-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/call-metadata/call-metadata.ll:10:8: note: possible intended match here
 %_1 = call i8 @some_true(), !range !2

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/call-metadata/call-metadata.ll
Check file: /checkout/src/test/codegen/call-metadata.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
           1: ; ModuleID = 'call_metadata.cd7be62b-cgu.0'
check:9'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           2: source_filename = "call_metadata.cd7be62b-cgu.0"
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           4: target triple = "x86_64-unknown-linux-gnu"
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           5: 
check:9'0     ~
           6: ; call_metadata::test
check:9'0     ~~~~~~~~~~~~~~~~~~~~~
           7: ; Function Attrs: nonlazybind uwtable
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           8: define void @_ZN13call_metadata4test17hb23c4e97eeec23c9E() unnamed_addr #0 {
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           9: start:
check:9'0     ~~~~~~
          10:  %_1 = call i8 @some_true(), !range !2
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:9'1            ?                               possible intended match
          11:  br label %bb1
check:9'0     ~~~~~~~~~~~~~~
          12: 
check:9'0     ~
          13: bb1: ; preds = %start
check:9'0     ~~~~~~~~~~~~~~~~~~~~~
          14:  ret void
check:9'0     ~~~~~~~~~
          15: }
check:9'0     ~
           .
           .
>>>>>>
------------------------------------------
------------------------------------------


---- [codegen] codegen/function-arguments.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments/function-arguments.ll" "/checkout/src/test/codegen/function-arguments.rs" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/function-arguments.rs:46:11: error: CHECK: expected string not found in input
// CHECK: noundef i32 @char(i32 noundef %x)
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments/function-arguments.ll:114:41: note: scanning from here
define i8 @maybeuninit_enum_bool(i8 %x) unnamed_addr #0 {
                                        ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments/function-arguments.ll:120:2: note: possible intended match here
define i32 @char(i32 %x) unnamed_addr #0 {

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments/function-arguments.ll
Check file: /checkout/src/test/codegen/function-arguments.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
          109: start:
          110:  ret i1 %x
          111: }
          112: 
          113: ; Function Attrs: nonlazybind uwtable
          114: define i8 @maybeuninit_enum_bool(i8 %x) unnamed_addr #0 {
check:46'0                                             X~~~~~~~~~~~~~~~~ error: no match found
          115: start:
check:46'0     ~~~~~~
          116:  ret i8 %x
check:46'0     ~~~~~~~~~~
          117: }
check:46'0     ~
          118: 
check:46'0     ~
          119: ; Function Attrs: nonlazybind uwtable
check:46'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          120: define i32 @char(i32 %x) unnamed_addr #0 {
check:46'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:46'1      ?                                         possible intended match
          121: start:
check:46'0     ~~~~~~
          122:  ret i32 %x
check:46'0     ~~~~~~~~~~~
          123: }
check:46'0     ~
          124: 
check:46'0     ~
          125: ; Function Attrs: nonlazybind uwtable
check:46'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
------------------------------------------


---- [codegen] codegen/repr-transparent.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repr-transparent/repr-transparent.ll" "/checkout/src/test/codegen/repr-transparent.rs" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/repr-transparent.rs:59:11: error: CHECK: expected string not found in input
// CHECK: define{{( dso_local)?}} noundef{{( zeroext)?}} i8 @test_Gpz(i8 noundef{{( zeroext)?}} %_1)
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repr-transparent/repr-transparent.ll:43:41: note: scanning from here
define double @test_Generic(double %_1) unnamed_addr #0 {

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repr-transparent/repr-transparent.ll
Check file: /checkout/src/test/codegen/repr-transparent.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
          .
          .
          .
          .
         38: bb1: ; preds = %bb1, %start
         39:  br label %bb1
         40: }
         41: 
         42: ; Function Attrs: nonlazybind uwtable
         43: define double @test_Generic(double %_1) unnamed_addr #0 {
check:59                                             X~~~~~~~~~~~~~~~~ error: no match found
check:59     ~~~~~~
check:59     ~~~~~~
         45:  br label %bb1
         46: 
check:59     ~
check:59     ~
         47: bb1: ; preds = %bb1, %start
check:59     ~~~~~~~~~~~~~~~~~~~~~~~~~~~
         48:  br label %bb1
          .
          .
          .
>>>>>>
