plain
failures:

---- [codegen] tests/codegen/drop-in-place-noalias.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/drop-in-place-noalias/drop-in-place-noalias.ll" "/checkout/tests/codegen/drop-in-place-noalias.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
/checkout/tests/codegen/drop-in-place-noalias.rs:10:11: error: CHECK: expected string not found in input
/checkout/tests/codegen/drop-in-place-noalias.rs:10:11: error: CHECK: expected string not found in input
// CHECK: define internal void @{{.*}}core{{.*}}ptr{{.*}}drop_in_place{{.*}}StructUnpin{{.*}}({{.*\*|ptr}} noalias noundef align 4 dereferenceable(12) %{{.+}})
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/drop-in-place-noalias/drop-in-place-noalias.ll:1:1: note: scanning from here
; ModuleID = 'drop_in_place_noalias.d3bd3bc72675f241-cgu.0'

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/drop-in-place-noalias/drop-in-place-noalias.ll
Check file: /checkout/tests/codegen/drop-in-place-noalias.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
          1: ; ModuleID = 'drop_in_place_noalias.d3bd3bc72675f241-cgu.0' 
check:10     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
          2: source_filename = "drop_in_place_noalias.d3bd3bc72675f241-cgu.0" 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
          4: target triple = "x86_64-unknown-linux-gnu" 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          5:  
check:10     ~
check:10     ~
          6: %StructNotUnpin = type { %"core::marker::PhantomPinned", i32, i32, i32 } 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          7: %"core::marker::PhantomPinned" = type {} 
          8:  
check:10     ~
          9: ; core::mem::drop 
check:10     ~~~~~~~~~~~~~~~~~~
check:10     ~~~~~~~~~~~~~~~~~~
         10: ; Function Attrs: inlinehint nonlazybind uwtable 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         11: define void @_ZN4core3mem4drop17h05fdca1e506da3d1E(ptr %_x) unnamed_addr #0 { 
         12: start: 
check:10     ~~~~~~~
check:10     ~~~~~~~
         13: ; call core::ptr::drop_in_place<drop_in_place_noalias::StructNotUnpin> 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         14:  call void @"_ZN4core3ptr58drop_in_place$LT$drop_in_place_noalias..StructNotUnpin$GT$17hd3c718e14765edf7E"(ptr align 4 %_x) 
         15:  ret void 
check:10     ~~~~~~~~~~
         16: } 
check:10     ~~
check:10     ~~
         17:  
check:10     ~
         18: ; core::mem::drop 
check:10     ~~~~~~~~~~~~~~~~~~
         19: ; Function Attrs: inlinehint nonlazybind uwtable 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         20: define void @_ZN4core3mem4drop17h0f863790d6a6cf30E(ptr %_x) unnamed_addr #0 { 
         21: start: 
check:10     ~~~~~~~
check:10     ~~~~~~~
         22: ; call core::ptr::drop_in_place<drop_in_place_noalias::StructUnpin> 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         23:  call void @"_ZN4core3ptr55drop_in_place$LT$drop_in_place_noalias..StructUnpin$GT$17h52883d5d0e79ee92E"(ptr align 4 %_x) 
         24:  ret void 
check:10     ~~~~~~~~~~
         25: } 
check:10     ~~
check:10     ~~
         26:  
check:10     ~
         27: ; core::ptr::drop_in_place<drop_in_place_noalias::StructUnpin> 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         28: ; Function Attrs: nonlazybind uwtable 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         29: define void @"_ZN4core3ptr55drop_in_place$LT$drop_in_place_noalias..StructUnpin$GT$17h52883d5d0e79ee92E"(ptr align 4 %_1) unnamed_addr #1 { 
         30: start: 
check:10     ~~~~~~~
check:10     ~~~~~~~
         31: ; call <drop_in_place_noalias::StructUnpin as core::ops::drop::Drop>::drop 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         32:  call void @"_ZN76_$LT$drop_in_place_noalias..StructUnpin$u20$as$u20$core..ops..drop..Drop$GT$4drop17hb92bfba86ceb236aE"(ptr align 4 %_1) 
         33:  ret void 
check:10     ~~~~~~~~~~
         34: } 
check:10     ~~
check:10     ~~
         35:  
check:10     ~
         36: ; core::ptr::drop_in_place<drop_in_place_noalias::StructNotUnpin> 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         37: ; Function Attrs: nonlazybind uwtable 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         38: define void @"_ZN4core3ptr58drop_in_place$LT$drop_in_place_noalias..StructNotUnpin$GT$17hd3c718e14765edf7E"(ptr align 4 %_1) unnamed_addr #1 { 
         39: start: 
check:10     ~~~~~~~
check:10     ~~~~~~~
         40: ; call <drop_in_place_noalias::StructNotUnpin as core::ops::drop::Drop>::drop 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         41:  call void @"_ZN79_$LT$drop_in_place_noalias..StructNotUnpin$u20$as$u20$core..ops..drop..Drop$GT$4drop17h95c50d8058676f4cE"(ptr align 4 %_1) 
         42:  ret void 
check:10     ~~~~~~~~~~
         43: } 
check:10     ~~
check:10     ~~
         44:  
check:10     ~
         45: ; <drop_in_place_noalias::StructUnpin as core::ops::drop::Drop>::drop 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         46: ; Function Attrs: nonlazybind uwtable 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         47: define void @"_ZN76_$LT$drop_in_place_noalias..StructUnpin$u20$as$u20$core..ops..drop..Drop$GT$4drop17hb92bfba86ceb236aE"(ptr align 4 %self) unnamed_addr #1 { 
         48: start: 
check:10     ~~~~~~~
         49:  ret void 
check:10     ~~~~~~~~~~
check:10     ~~~~~~~~~~
         50: } 
check:10     ~~
         51:  
check:10     ~
         52: ; <drop_in_place_noalias::StructNotUnpin as core::ops::drop::Drop>::drop 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         53: ; Function Attrs: nonlazybind uwtable 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         54: define void @"_ZN79_$LT$drop_in_place_noalias..StructNotUnpin$u20$as$u20$core..ops..drop..Drop$GT$4drop17h95c50d8058676f4cE"(ptr align 4 %self) unnamed_addr #1 { 
         55: start: 
check:10     ~~~~~~~
         56:  ret void 
check:10     ~~~~~~~~~~
check:10     ~~~~~~~~~~
         57: } 
check:10     ~~
         58:  
check:10     ~
         59: ; drop_in_place_noalias::main 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         60: ; Function Attrs: nonlazybind uwtable 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         61: define void @_ZN21drop_in_place_noalias4main17h5ec73c3e72d0e5bbE(ptr %x, ptr %y) unnamed_addr #1 personality ptr @rust_eh_personality { 
         62: start: 
check:10     ~~~~~~~
check:10     ~~~~~~~
         63:  %0 = alloca { ptr, i32 }, align 8 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         64:  %_6 = alloca i8, align 1 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~
         65:  %_5 = alloca %StructNotUnpin, align 4 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         66:  store i8 0, ptr %_6, align 1 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         67:  store i8 1, ptr %_6, align 1 
         68: ; invoke core::mem::drop 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~
         69:  invoke void @_ZN4core3mem4drop17h0f863790d6a6cf30E(ptr %x) 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         70:  to label %bb1 unwind label %cleanup 
         71:  
check:10     ~
check:10     ~
         72: bb5: ; preds = %cleanup 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~
         73:  %1 = load i8, ptr %_6, align 1, !range !2, !noundef !3 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         74:  %2 = trunc i8 %1 to i1 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~
         75:  br i1 %2, label %bb4, label %bb3 
         76:  
check:10     ~
check:10     ~
         77: cleanup: ; preds = %bb1, %start 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         78:  %3 = landingpad { ptr, i32 } 
         79:  cleanup 
check:10     ~~~~~~~~~
check:10     ~~~~~~~~~
         80:  %4 = extractvalue { ptr, i32 } %3, 0 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         81:  %5 = extractvalue { ptr, i32 } %3, 1 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         82:  %6 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 0 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         83:  store ptr %4, ptr %6, align 8 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         84:  %7 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 1 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         85:  store i32 %5, ptr %7, align 8 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         86:  br label %bb5 
         87:  
check:10     ~
check:10     ~
         88: bb1: ; preds = %start 
check:10     ~~~~~~~~~~~~~~~~~~~~~~
         89:  store i8 0, ptr %_6, align 1 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         90:  call void @llvm.memcpy.p0.p0.i64(ptr align 4 %_5, ptr align 4 %y, i64 12, i1 false) 
         91: ; invoke core::mem::drop 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~
         92:  invoke void @_ZN4core3mem4drop17h05fdca1e506da3d1E(ptr %_5) 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         93:  to label %bb2 unwind label %cleanup 
         94:  
check:10     ~
check:10     ~
         95: bb2: ; preds = %bb1 
         96:  ret void 
check:10     ~~~~~~~~~~
         97:  
check:10     ~
check:10     ~
         98: bb3: ; preds = %bb4, %bb5 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~
         99:  %8 = load ptr, ptr %0, align 8, !noundef !3 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        100:  %9 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 1 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        101:  %10 = load i32, ptr %9, align 8, !noundef !3 
          .
          .
          .
>>>>>>
