plain
test [codegen] src/test/codegen/lto-removes-invokes.rs ... ok

failures:

---- [codegen] src/test/codegen/align-byval.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/align-byval/align-byval.ll" "/checkout/src/test/codegen/align-byval.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=arm-linux-androideabi
--- stderr -------------------------------
/checkout/src/test/codegen/align-byval.rs:14:12: error: CHECK: expected string not found in input
 // CHECK: declare void @f({{.*}}byval(%Foo) align 16{{.*}})
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/align-byval/align-byval.ll:1:1: note: scanning from here
; ModuleID = 'align_byval.33f0fd8f-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/align-byval/align-byval.ll:73:1: note: possible intended match here
declare void @f([8 x i64]) unnamed_addr #1


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/align-byval/align-byval.ll
Check file: /checkout/src/test/codegen/align-byval.rs

-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'align_byval.33f0fd8f-cgu.0' 
check:14'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            2: source_filename = "align_byval.33f0fd8f-cgu.0" 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            3: target datalayout = "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64" 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            4: target triple = "arm-unknown-linux-android" 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            5:  
check:14'0     ~
            6: @vtable.0 = private unnamed_addr constant <{ ptr, [8 x i8], ptr, ptr, ptr }> <{ ptr @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17hb69cd3bf9e396fa4E", [8 x i8] c"\04\00\00\00\04\00\00\00", ptr @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h77818e13d33910d9E", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h883f87dcc37b43ccE", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h883f87dcc37b43ccE" }>, align 4 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
           68: ; std::rt::lang_start_internal 
           68: ; std::rt::lang_start_internal 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           69: ; Function Attrs: nonlazybind uwtable 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           70: declare i32 @_ZN3std2rt19lang_start_internal17h9eafcd719240241eE(ptr noundef nonnull align 1, ptr noalias noundef readonly align 4 dereferenceable(12), i32, ptr, i8) unnamed_addr #1 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           71:  
check:14'0     ~
           72: ; Function Attrs: nonlazybind uwtable 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           73: declare void @f([8 x i64]) unnamed_addr #1 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:14'1     ?                                           possible intended match
           74:  
check:14'0     ~
           75: ; Function Attrs: nonlazybind 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           76: define i32 @main(i32 %0, ptr %1) unnamed_addr #4 { 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           77: top: 
check:14'0     ~~~~~
           78:  %_9.i = alloca ptr, align 4 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
