plain
failures:

---- [codegen] src/test/codegen/abi-main-signature-32bit-c-int.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-main-signature-32bit-c-int/abi-main-signature-32bit-c-int.ll" "/checkout/src/test/codegen/abi-main-signature-32bit-c-int.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/abi-main-signature-32bit-c-int.rs:10:11: error: CHECK: expected string not found in input
// CHECK: define i32 @main(i32{{( %0)?}}, {{i8\*\*|ptr}}{{( %1)?}})
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-main-signature-32bit-c-int/abi-main-signature-32bit-c-int.ll:1:1: note: scanning from here
; ModuleID = 'abi_main_signature_32bit_c_int.1284ba6c-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-main-signature-32bit-c-int/abi-main-signature-32bit-c-int.ll:71:11: note: possible intended match here
define hidden i32 @main(i32 %0, ptr %1) unnamed_addr #5 {

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-main-signature-32bit-c-int/abi-main-signature-32bit-c-int.ll
Check file: /checkout/src/test/codegen/abi-main-signature-32bit-c-int.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'abi_main_signature_32bit_c_int.1284ba6c-cgu.0' 
check:10'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            2: source_filename = "abi_main_signature_32bit_c_int.1284ba6c-cgu.0" 
check:10'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            3: target datalayout = "e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-f128:64-n32:64-S128-ni:1:10:20" 
check:10'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            4: target triple = "wasm32-unknown-emscripten" 
check:10'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            5:  
check:10'0     ~
            6: @vtable.0 = private unnamed_addr constant <{ ptr, [8 x i8], ptr, ptr, ptr }> <{ ptr @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h5fa8977f25133f8aE", [8 x i8] c"\04\00\00\00\04\00\00\00", ptr @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hda6a54458b2e61afE", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hbe7b2f4041de1bfdE", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hbe7b2f4041de1bfdE" }>, align 4 
check:10'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
           66:  
           66:  
check:10'0     ~
           67: ; std::rt::lang_start_internal 
check:10'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           68: ; Function Attrs: uwtable 
check:10'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
           69: declare i32 @_ZN3std2rt19lang_start_internal17ha16dd3f52aaf86beE(ptr noundef nonnull align 1, ptr noalias noundef readonly align 4 dereferenceable(12), i32, ptr, i8) unnamed_addr #1 
check:10'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           70:  
check:10'0     ~
           71: define hidden i32 @main(i32 %0, ptr %1) unnamed_addr #5 { 
check:10'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:10'1               ?                                                possible intended match
           72: top: 
check:10'0     ~~~~~
           73:  %_9.i = alloca ptr, align 4 
check:10'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           74:  call void @llvm.lifetime.start.p0(i64 4, ptr nonnull %_9.i) 
check:10'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           75:  store ptr @_ZN30abi_main_signature_32bit_c_int4main17h5a3a1f8d9c296a4fE, ptr %_9.i, align 4 
check:10'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           76: ; call std::rt::lang_start_internal 
check:10'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
