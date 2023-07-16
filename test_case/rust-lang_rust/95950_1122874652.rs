plain
failures:

---- [codegen] src/test/codegen/call-metadata.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/call-metadata/call-metadata.ll" "/checkout/src/test/codegen/call-metadata.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/call-metadata.rs:9:12: error: CHECK: expected string not found in input
 // CHECK: call noundef i8 @some_true(), !range [[R0:![0-9]+]]
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/call-metadata/call-metadata.ll:1:1: note: scanning from here
; ModuleID = 'call_metadata.cd7be62b-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/call-metadata/call-metadata.ll:10:8: note: possible intended match here
 %_1 = call noundef i8 @some_true() #1, !range !0

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/call-metadata/call-metadata.ll
Check file: /checkout/src/test/codegen/call-metadata.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
           1: ; ModuleID = 'call_metadata.cd7be62b-cgu.0' 
check:9'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           2: source_filename = "call_metadata.cd7be62b-cgu.0" 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           3: target datalayout = "e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-f128:64-n32:64-S128-ni:1:10:20" 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           4: target triple = "wasm32-unknown-emscripten" 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           5:  
check:9'0     ~
           6: ; call_metadata::test 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~
           7: ; Function Attrs: nounwind 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~
           8: define hidden void @_ZN13call_metadata4test17h0d500cb667ecfe03E() unnamed_addr #0 { 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           9: start: 
check:9'0     ~~~~~~~
          10:  %_1 = call noundef i8 @some_true() #1, !range !0 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:9'1            ?                                           possible intended match
          11:  br label %bb1 
check:9'0     ~~~~~~~~~~~~~~~
          12:  
check:9'0     ~
          13: bb1: ; preds = %start 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~
          14:  ret void 
check:9'0     ~~~~~~~~~~
          15: } 
check:9'0     ~~
           .
           .
>>>>>>
------------------------------------------
------------------------------------------


---- [codegen] src/test/codegen/debug-column.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/debug-column/debug-column.ll" "/checkout/src/test/codegen/debug-column.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/debug-column.rs:9:12: error: CHECK: expected string not found in input
 // CHECK: call void @giraffe(), !dbg [[A:!.*]]
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/debug-column/debug-column.ll:1:1: note: scanning from here
; ModuleID = 'debug_column.30acdd43-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/debug-column/debug-column.ll:16:7: note: possible intended match here
 tail call void %f() #7, !dbg !58

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/debug-column/debug-column.ll
Check file: /checkout/src/test/codegen/debug-column.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
           1: ; ModuleID = 'debug_column.30acdd43-cgu.0' 
check:9'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           2: source_filename = "debug_column.30acdd43-cgu.0" 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           3: target datalayout = "e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-f128:64-n32:64-S128-ni:1:10:20" 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           4: target triple = "wasm32-unknown-emscripten" 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           5:  
check:9'0     ~
           6: @vtable.0 = private unnamed_addr constant <{ i8*, [8 x i8], i8*, i8*, i8* }> <{ i8* bitcast (void (i32**)* @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h362e73e5a1fc4402E" to i8*), [8 x i8] c"\04\00\00\00\04\00\00\00", i8* bitcast (i32 (i32**)* @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hed54dc78a51532cdE" to i8*), i8* bitcast (i32 (i32**)* @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hc73f2279eb30f5c6E" to i8*), i8* bitcast (i32 (i32**)* @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hc73f2279eb30f5c6E" to i8*) }>, align 4, !dbg !0 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           .
           .
          11: start: 
          11: start: 
check:9'0     ~~~~~~~
          12:  call void @llvm.dbg.declare(metadata {}* undef, metadata !37, metadata !DIExpression()), !dbg !42 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          13:  call void @llvm.dbg.value(metadata void ()* %f, metadata !36, metadata !DIExpression()), !dbg !43 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          14:  call void @llvm.dbg.value(metadata void ()* %f, metadata !44, metadata !DIExpression()) #7, !dbg !56 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          15:  call void @llvm.dbg.declare(metadata {}* undef, metadata !52, metadata !DIExpression()) #7, !dbg !58 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          16:  tail call void %f() #7, !dbg !58 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:9'1           ?                            possible intended match
          17:  call void @llvm.dbg.declare(metadata {}* undef, metadata !59, metadata !DIExpression()) #7, !dbg !67 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          18:  tail call void asm sideeffect "", "r,~{memory}"({}* undef) #7, !dbg !69, !srcloc !70 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          19:  ret void, !dbg !71 
check:9'0     ~~~~~~~~~~~~~~~~~~~~
          20: } 
check:9'0     ~~
          21:  
check:9'0     ~
           .
           .
>>>>>>
------------------------------------------
