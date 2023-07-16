plain
failures:

---- [codegen] codegen/function-arguments-noopt.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments-noopt/function-arguments-noopt.ll" "/checkout/src/test/codegen/function-arguments-noopt.rs" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/function-arguments-noopt.rs:13:11: error: CHECK: expected string not found in input
// CHECK: zeroext i1 @boolean(i1 zeroext %x)
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments-noopt/function-arguments-noopt.ll:1:1: note: scanning from here
; ModuleID = 'function_arguments_noopt.8beba419-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments-noopt/function-arguments-noopt.ll:9:16: note: possible intended match here
define noundef zeroext i1 @boolean(i1 noundef zeroext %x) unnamed_addr #0 {
               ^
/checkout/src/test/codegen/function-arguments-noopt.rs:22:11: error: CHECK: expected string not found in input
// CHECK: call zeroext i1 %f(i1 zeroext %x)
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments-noopt/function-arguments-noopt.ll:15:40: note: scanning from here
define noundef zeroext i1 @boolean_call(i1 noundef zeroext %x, i1 (i1)* noundef nonnull %f) unnamed_addr #0 {
                                       ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments-noopt/function-arguments-noopt.ll:17:15: note: possible intended match here
 %0 = call noundef zeroext i1 %f(i1 noundef zeroext %x)
              ^
/checkout/src/test/codegen/function-arguments-noopt.rs:35:12: error: CHECK: expected string not found in input
 // CHECK: call align 4 i32* %f(i32* align 4 %x)
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments-noopt/function-arguments-noopt.ll:31:60: note: scanning from here
define noundef align 4 dereferenceable(4) i32* @borrow_call(i32* noundef align 4 dereferenceable(4) %x, i32* (i32*)* noundef nonnull %f) unnamed_addr #0 {
                                                           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments-noopt/function-arguments-noopt.ll:45:33: note: possible intended match here
 call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %1, i8* align 4 %2, i64 32, i1 false)
                                ^
/checkout/src/test/codegen/function-arguments-noopt.rs:48:12: error: CHECK: expected string not found in input
 // CHECK: call void %f(%S* sret(%S){{( %0)?}}, %S* %{{.+}})
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments-noopt/function-arguments-noopt.ll:50:25: note: scanning from here
define void @struct_call(%S* noalias nocapture noundef sret(%S) dereferenceable(32) %0, %S* noalias nocapture noundef dereferenceable(32) %x, void (%S*, %S*)* noundef nonnull %f) unnamed_addr #0 {
                        ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments-noopt/function-arguments-noopt.ll:56:2: note: possible intended match here
 call void %f(%S* noalias nocapture noundef sret(%S) dereferenceable(32) %0, %S* noalias nocapture noundef dereferenceable(32) %_4)
 ^
/checkout/src/test/codegen/function-arguments-noopt.rs:61:12: error: CHECK: expected string not found in input
 // CHECK: call { i8, i8 } %f(i1 zeroext %x.0, i8 %x.1)
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments-noopt/function-arguments-noopt.ll:73:29: note: scanning from here
define { i8, i8 } @enum_call(i1 noundef zeroext %x.0, i8 %x.1, { i8, i8 } (i1, i8)* noundef nonnull %f) unnamed_addr #0 {
                            ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments-noopt/function-arguments-noopt.ll:75:7: note: possible intended match here
 %0 = call { i8, i8 } %f(i1 noundef zeroext %x.0, i8 %x.1)

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments-noopt/function-arguments-noopt.ll
Check file: /checkout/src/test/codegen/function-arguments-noopt.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'function_arguments_noopt.8beba419-cgu.0'
check:13'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            2: source_filename = "function_arguments_noopt.8beba419-cgu.0"
check:13'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
check:13'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            4: target triple = "x86_64-unknown-linux-gnu"
check:13'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            5: 
check:13'0     ~
            6: %S = type { [8 x i32] }
check:13'0     ~~~~~~~~~~~~~~~~~~~~~~~
            7: 
check:13'0     ~
            8: ; Function Attrs: nonlazybind uwtable
check:13'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            9: define noundef zeroext i1 @boolean(i1 noundef zeroext %x) unnamed_addr #0 {
check:13'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:13'1                    ?                                                            possible intended match
           10: start:
check:13'0     ~~~~~~
           11:  ret i1 %x
check:13'0     ~~~~~~~~~~
           12: }
check:13'0     ~
           13: 
check:13'0     ~
           14: ; Function Attrs: nonlazybind uwtable
check:13'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           15: define noundef zeroext i1 @boolean_call(i1 noundef zeroext %x, i1 (i1)* noundef nonnull %f) unnamed_addr #0 {
check:13'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:22'0                                            X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           16: start:
check:22'0     ~~~~~~
           17:  %0 = call noundef zeroext i1 %f(i1 noundef zeroext %x)
check:22'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:22'1                   ?                                         possible intended match
           18:  br label %bb1
check:22'0     ~~~~~~~~~~~~~~
           19: 
check:22'0     ~
           20: bb1: ; preds = %start
check:22'0     ~~~~~~~~~~~~~~~~~~~~~
           21:  ret i1 %0
check:22'0     ~~~~~~~~~~
           22: }
check:22'0     ~
            .
            .
           26: start:
           26: start:
check:22'0     ~~~~~~
           27:  ret i32* %x
check:22'0     ~~~~~~~~~~~~
           28: }
check:22'0     ~
           29: 
check:22'0     ~
           30: ; Function Attrs: nonlazybind uwtable
check:22'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           31: define noundef align 4 dereferenceable(4) i32* @borrow_call(i32* noundef align 4 dereferenceable(4) %x, i32* (i32*)* noundef nonnull %f) unnamed_addr #0 {
check:22'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:35'0                                                                X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           32: start:
check:35'0     ~~~~~~
           33:  %0 = call noundef align 4 dereferenceable(4) i32* %f(i32* noundef align 4 dereferenceable(4) %x)
check:35'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           34:  br label %bb1
check:35'0     ~~~~~~~~~~~~~~
           35: 
check:35'0     ~
           36: bb1: ; preds = %start
check:35'0     ~~~~~~~~~~~~~~~~~~~~~
            .
            .
            .
           40: ; Function Attrs: nonlazybind uwtable
check:35'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           41: define void @struct_(%S* noalias nocapture noundef sret(%S) dereferenceable(32) %0, %S* noalias nocapture noundef dereferenceable(32) %x) unnamed_addr #0 {
check:35'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           42: start:
check:35'0     ~~~~~~
           43:  %1 = bitcast %S* %0 to i8*
check:35'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~
           44:  %2 = bitcast %S* %x to i8*
check:35'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~
           45:  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %1, i8* align 4 %2, i64 32, i1 false)
check:35'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:35'1                                     ?                                                       possible intended match
           46:  ret void
check:35'0     ~~~~~~~~~
           47: }
check:35'0     ~
           48: 
check:35'0     ~
           49: ; Function Attrs: nonlazybind uwtable
check:35'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           50: define void @struct_call(%S* noalias nocapture noundef sret(%S) dereferenceable(32) %0, %S* noalias nocapture noundef dereferenceable(32) %x, void (%S*, %S*)* noundef nonnull %f) unnamed_addr #0 {
check:35'0     ~~~~~~~~~~~~~~~~~~~~~~~~
check:48'0                             X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           51: start:
check:48'0     ~~~~~~
           52:  %_4 = alloca %S, align 4
check:48'0     ~~~~~~~~~~~~~~~~~~~~~~~~~
           53:  %1 = bitcast %S* %_4 to i8*
check:48'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           54:  %2 = bitcast %S* %x to i8*
check:48'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~
           55:  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %1, i8* align 4 %2, i64 32, i1 false)
check:48'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           56:  call void %f(%S* noalias nocapture noundef sret(%S) dereferenceable(32) %0, %S* noalias nocapture noundef dereferenceable(32) %_4)
check:48'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:48'1      ?                                                                                                                                  possible intended match
           57:  br label %bb1
check:48'0     ~~~~~~~~~~~~~~
           58: 
check:48'0     ~
           59: bb1: ; preds = %start
check:48'0     ~~~~~~~~~~~~~~~~~~~~~
           60:  ret void
check:48'0     ~~~~~~~~~
           61: }
check:48'0     ~
            .
            .
            .
           68:  %2 = insertvalue { i8, i8 } %1, i8 %x.1, 1
check:48'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           69:  ret { i8, i8 } %2
check:48'0     ~~~~~~~~~~~~~~~~~~
           70: }
check:48'0     ~
           71: 
check:48'0     ~
           72: ; Function Attrs: nonlazybind uwtable
check:48'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           73: define { i8, i8 } @enum_call(i1 noundef zeroext %x.0, i8 %x.1, { i8, i8 } (i1, i8)* noundef nonnull %f) unnamed_addr #0 {
check:48'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:61'0                                 X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           74: start:
check:61'0     ~~~~~~
           75:  %0 = call { i8, i8 } %f(i1 noundef zeroext %x.0, i8 %x.1)
check:61'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:61'1           ?                                                    possible intended match
           76:  %1 = extractvalue { i8, i8 } %0, 0
check:61'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           77:  %2 = trunc i8 %1 to i1
check:61'0     ~~~~~~~~~~~~~~~~~~~~~~~
           78:  %3 = extractvalue { i8, i8 } %0, 1
check:61'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           79:  br label %bb1
check:61'0     ~~~~~~~~~~~~~~
           80: 
check:61'0     ~
            .
            .
>>>>>>
------------------------------------------
