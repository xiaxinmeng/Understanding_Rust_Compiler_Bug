plain
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=wasm32-unknown-emscripten

failures:

---- [codegen] codegen/function-arguments-noopt.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments-noopt/function-arguments-noopt.ll" "/checkout/src/test/codegen/function-arguments-noopt.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/function-arguments-noopt.rs:13:11: error: CHECK: expected string not found in input
// CHECK: define zeroext i1 @boolean(i1 zeroext %x)
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments-noopt/function-arguments-noopt.ll:1:1: note: scanning from here
; ModuleID = 'function_arguments_noopt.8beba419-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments-noopt/function-arguments-noopt.ll:9:11: note: possible intended match here
define dso_local zeroext i1 @boolean(i1 zeroext %x) unnamed_addr #0 {
          ^
/checkout/src/test/codegen/function-arguments-noopt.rs:26:11: error: CHECK: expected string not found in input
// CHECK: define align 4 i32* @borrow(i32* align 4 %x)
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments-noopt/function-arguments-noopt.ll:17:40: note: scanning from here
 %0 = call zeroext i1 %f(i1 zeroext %x)
                                       ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments-noopt/function-arguments-noopt.ll:25:11: note: possible intended match here
define dso_local align 4 i32* @borrow(i32* align 4 %x) unnamed_addr #0 {
          ^
/checkout/src/test/codegen/function-arguments-noopt.rs:39:11: error: CHECK: expected string not found in input
// CHECK: define void @struct_(%S* sret(%S){{( %0)?}}, %S* %x)
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments-noopt/function-arguments-noopt.ll:33:44: note: scanning from here
 %0 = call align 4 i32* %f(i32* align 4 %x)
                                           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments-noopt/function-arguments-noopt.ll:41:1: note: possible intended match here
define dso_local void @struct_(%S* sret(%S) %0, %S* %x) unnamed_addr #0 {
^
/checkout/src/test/codegen/function-arguments-noopt.rs:52:11: error: CHECK: expected string not found in input
// CHECK: define { i8, i8 } @enum_(i1 zeroext %x.0, i8 %x.1)
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments-noopt/function-arguments-noopt.ll:56:40: note: scanning from here
 call void %f(%S* sret(%S) %0, %S* %_4)
                                       ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments-noopt/function-arguments-noopt.ll:64:11: note: possible intended match here
define dso_local { i8, i8 } @enum_(i1 zeroext %x.0, i8 %x.1) unnamed_addr #0 {


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments-noopt/function-arguments-noopt.ll
Check file: /checkout/src/test/codegen/function-arguments-noopt.rs

-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'function_arguments_noopt.8beba419-cgu.0' 
check:13'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            2: source_filename = "function_arguments_noopt.8beba419-cgu.0" 
check:13'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            3: target datalayout = "e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-f128:64-n32:64-S128-ni:1:10:20" 
check:13'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            4: target triple = "wasm32-unknown-emscripten" 
check:13'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            5:  
check:13'0     ~
            6: %S = type { [8 x i32] } 
check:13'0     ~~~~~~~~~~~~~~~~~~~~~~~~
            7:  
check:13'0     ~
            8: ; Function Attrs: uwtable 
check:13'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
            9: define dso_local zeroext i1 @boolean(i1 zeroext %x) unnamed_addr #0 { 
check:13'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:13'1               ?                                                            possible intended match
           10: start: 
check:13'0     ~~~~~~~
           11:  ret i1 %x 
check:13'0     ~~~~~~~~~~~
           12: } 
check:13'0     ~~
           13:  
check:13'0     ~
           14: ; Function Attrs: uwtable 
check:13'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
           15: define dso_local zeroext i1 @boolean_call(i1 zeroext %x, i1 (i1)* %f) unnamed_addr #0 { 
check:13'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           16: start: 
           17:  %0 = call zeroext i1 %f(i1 zeroext %x) 
check:26'0                                            X error: no match found
           18:  br label %bb1 
check:26'0     ~~~~~~~~~~~~~~~
           19:  
check:26'0     ~
           20: bb1: ; preds = %start 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~
           21:  ret i1 %0 
check:26'0     ~~~~~~~~~~~
           22: } 
check:26'0     ~~
           23:  
check:26'0     ~
           24: ; Function Attrs: uwtable 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
           25: define dso_local align 4 i32* @borrow(i32* align 4 %x) unnamed_addr #0 { 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:26'1               ?                                                               possible intended match
           26: start: 
check:26'0     ~~~~~~~
           27:  ret i32* %x 
check:26'0     ~~~~~~~~~~~~~
           28: } 
check:26'0     ~~
           29:  
check:26'0     ~
           30: ; Function Attrs: uwtable 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
           31: define dso_local align 4 i32* @borrow_call(i32* align 4 %x, i32* (i32*)* %f) unnamed_addr #0 { 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           32: start: 
           33:  %0 = call align 4 i32* %f(i32* align 4 %x) 
check:39'0                                                X error: no match found
           34:  br label %bb1 
check:39'0     ~~~~~~~~~~~~~~~
           35:  
check:39'0     ~
           36: bb1: ; preds = %start 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~
           37:  ret i32* %0 
check:39'0     ~~~~~~~~~~~~~
           38: } 
check:39'0     ~~
           39:  
check:39'0     ~
           40: ; Function Attrs: uwtable 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
           41: define dso_local void @struct_(%S* sret(%S) %0, %S* %x) unnamed_addr #0 { 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:39'1     ?                                                                          possible intended match
           42: start: 
check:39'0     ~~~~~~~
           43:  %1 = bitcast %S* %0 to i8* 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           44:  %2 = bitcast %S* %x to i8* 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           45:  call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 4 %1, i8* align 4 %2, i32 32, i1 false) 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           46:  ret void 
check:39'0     ~~~~~~~~~~
            .
            .
           51: start: 
           51: start: 
           52:  %_4 = alloca %S, align 4 
           53:  %1 = bitcast %S* %_4 to i8* 
           54:  %2 = bitcast %S* %x to i8* 
           55:  call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 4 %1, i8* align 4 %2, i32 32, i1 false) 
           56:  call void %f(%S* sret(%S) %0, %S* %_4) 
check:52'0                                            X error: no match found
           57:  br label %bb1 
check:52'0     ~~~~~~~~~~~~~~~
           58:  
check:52'0     ~
           59: bb1: ; preds = %start 
check:52'0     ~~~~~~~~~~~~~~~~~~~~~~
           60:  ret void 
check:52'0     ~~~~~~~~~~
           61: } 
check:52'0     ~~
           62:  
check:52'0     ~
           63: ; Function Attrs: uwtable 
check:52'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
           64: define dso_local { i8, i8 } @enum_(i1 zeroext %x.0, i8 %x.1) unnamed_addr #0 { 
check:52'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:52'1               ?                                                                     possible intended match
           65: start: 
check:52'0     ~~~~~~~
           66:  %0 = zext i1 %x.0 to i8 
check:52'0     ~~~~~~~~~~~~~~~~~~~~~~~~~
           67:  %1 = insertvalue { i8, i8 } undef, i8 %0, 0 
check:52'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           68:  %2 = insertvalue { i8, i8 } %1, i8 %x.1, 1 
check:52'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           69:  ret { i8, i8 } %2 
check:52'0     ~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
