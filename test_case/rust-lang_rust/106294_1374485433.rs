plain
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [codegen] src/test/codegen/abi-sysv64.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-sysv64/abi-sysv64.ll" "/checkout/src/test/codegen/abi-sysv64.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/abi-sysv64.rs:18:11: error: CHECK: expected string not found in input
// CHECK: define x86_64_sysvcc noundef i64 @has_sysv64_abi
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-sysv64/abi-sysv64.ll:1:1: note: scanning from here
; ModuleID = 'abi_sysv64.f275fd3a-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-sysv64/abi-sysv64.ll:7:1: note: possible intended match here
define x86_64_sysvcc i64 @has_sysv64_abi(i64 %a) unnamed_addr #0 {

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-sysv64/abi-sysv64.ll
Check file: /checkout/src/test/codegen/abi-sysv64.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'abi_sysv64.f275fd3a-cgu.0' 
check:18'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            2: source_filename = "abi_sysv64.f275fd3a-cgu.0" 
check:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
check:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            4: target triple = "x86_64-unknown-linux-gnu" 
check:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            5:  
check:18'0     ~
            6: ; Function Attrs: nonlazybind uwtable 
check:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            7: define x86_64_sysvcc i64 @has_sysv64_abi(i64 %a) unnamed_addr #0 { 
check:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:18'1     ?                                                                   possible intended match
            8: start: 
check:18'0     ~~~~~~~
            9:  ret i64 %a 
check:18'0     ~~~~~~~~~~~~
           10: } 
check:18'0     ~~
           11:  
check:18'0     ~
           12: attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
check:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           13:  
check:18'0     ~
           14: !llvm.module.flags = !{!0, !1} 
check:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           15:  
check:18'0     ~
           16: !0 = !{i32 7, !"PIC Level", i32 2} 
check:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           17: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
check:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
------------------------------------------


---- [codegen] src/test/codegen/abi-x86-interrupt.rs stdout ----
---- [codegen] src/test/codegen/abi-x86-interrupt.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-x86-interrupt/abi-x86-interrupt.ll" "/checkout/src/test/codegen/abi-x86-interrupt.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/abi-x86-interrupt.rs:18:11: error: CHECK: expected string not found in input
// CHECK: define x86_intrcc noundef i64 @has_x86_interrupt_abi
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-x86-interrupt/abi-x86-interrupt.ll:1:1: note: scanning from here
; ModuleID = 'abi_x86_interrupt.246fd9bc-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-x86-interrupt/abi-x86-interrupt.ll:7:1: note: possible intended match here
define x86_intrcc i64 @has_x86_interrupt_abi(ptr byval(i64) %a) unnamed_addr #0 {


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-x86-interrupt/abi-x86-interrupt.ll


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'abi_x86_interrupt.246fd9bc-cgu.0' 
check:18'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            2: source_filename = "abi_x86_interrupt.246fd9bc-cgu.0" 
check:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
check:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            4: target triple = "x86_64-unknown-linux-gnu" 
check:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            5:  
check:18'0     ~
            6: ; Function Attrs: nounwind nonlazybind uwtable 
check:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            7: define x86_intrcc i64 @has_x86_interrupt_abi(ptr byval(i64) %a) unnamed_addr #0 { 
check:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:18'1     ?                                                                                  possible intended match
            8: start: 
check:18'0     ~~~~~~~
            9:  %0 = load i64, ptr %a, align 8, !noundef !2 
check:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           10:  ret i64 %0 
check:18'0     ~~~~~~~~~~~~
           11: } 
check:18'0     ~~
           12:  
check:18'0     ~
           13: attributes #0 = { nounwind nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
check:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           14:  
check:18'0     ~
           15: !llvm.module.flags = !{!0, !1} 
check:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           16:  
check:18'0     ~
           17: !0 = !{i32 7, !"PIC Level", i32 2} 
check:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           18: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
check:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           19: !2 = !{} 
check:18'0     ~~~~~~~~~
------------------------------------------


---- [codegen] src/test/codegen/adjustments.rs stdout ----
---- [codegen] src/test/codegen/adjustments.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/adjustments/adjustments.ll" "/checkout/src/test/codegen/adjustments.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/adjustments.rs:6:11: error: CHECK: expected string not found in input
// CHECK: @helper([[USIZE:i[0-9]+]] noundef %_1)
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/adjustments/adjustments.ll:1:1: note: scanning from here
; ModuleID = 'adjustments.7749b467-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/adjustments/adjustments.ll:7:13: note: possible intended match here
define void @helper(i64 %_1) unnamed_addr #0 {
/checkout/src/test/codegen/adjustments.rs:16:54: error: undefined variable: USIZE
/checkout/src/test/codegen/adjustments.rs:16:54: error: undefined variable: USIZE
// CHECK: %0 = insertvalue { {{\[0 x i8\]\*|ptr}}, [[USIZE]] } undef, {{\[0 x i8\]\*|ptr}} %x.0, 0
                                                     ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/adjustments/adjustments.ll:15:2: note: possible intended match here
 %0 = insertvalue { ptr, i64 } undef, ptr %x.0, 0

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/adjustments/adjustments.ll
Check file: /checkout/src/test/codegen/adjustments.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'adjustments.7749b467-cgu.0' 
check:6'0      X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            2: source_filename = "adjustments.7749b467-cgu.0" 
check:6'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
check:6'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            4: target triple = "x86_64-unknown-linux-gnu" 
check:6'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            5:  
check:6'0      ~
            6: ; Function Attrs: nonlazybind uwtable 
check:6'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            7: define void @helper(i64 %_1) unnamed_addr #0 { 
check:6'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:6'1                  ?                                   possible intended match
            8: start: 
check:6'0      ~~~~~~~
            9:  ret void 
check:6'0      ~~~~~~~~~~
           10: } 
check:6'0      ~~
           11:  
check:6'0      ~
           12: ; Function Attrs: nonlazybind uwtable 
check:6'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           13: define { ptr, i64 } @no_op_slice_adjustment(ptr align 1 %x.0, i64 %x.1) unnamed_addr #0 { 
check:6'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:16'0                                                X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: match failed for invalid pattern
check:16'1                                                                                                undefined variable: USIZE
           14: start: 
check:16'0     ~~~~~~~
           15:  %0 = insertvalue { ptr, i64 } undef, ptr %x.0, 0 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:16'2      ?                                                 possible intended match
           16:  %1 = insertvalue { ptr, i64 } %0, i64 %x.1, 1 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           17:  ret { ptr, i64 } %1 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~
           18: } 
check:16'0     ~~
           19:  
check:16'0     ~
           20: ; Function Attrs: nonlazybind uwtable 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           21: define { ptr, i64 } @no_op_slice_adjustment2(ptr align 1 %x.0, i64 %x.1) unnamed_addr #0 { 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           22: start: 
           23:  %0 = call { ptr, i64 } @no_op_slice_adjustment(ptr align 1 %x.0, i64 %x.1) 
           24:  %1 = extractvalue { ptr, i64 } %0, 0 
           25:  %2 = extractvalue { ptr, i64 } %0, 1 
           26:  %3 = insertvalue { ptr, i64 } undef, ptr %1, 0 
           27:  %4 = insertvalue { ptr, i64 } %3, i64 %2, 1 
           28:  ret { ptr, i64 } %4 
           29: } 
           30:  
           31: attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
           32:  
           33: !llvm.module.flags = !{!0, !1} 
           34:  
           35: !0 = !{i32 7, !"PIC Level", i32 2} 
           36: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
------------------------------------------


---- [codegen] src/test/codegen/call-llvm-intrinsics.rs stdout ----
---- [codegen] src/test/codegen/call-llvm-intrinsics.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/call-llvm-intrinsics/call-llvm-intrinsics.ll" "/checkout/src/test/codegen/call-llvm-intrinsics.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/call-llvm-intrinsics.rs:26:12: error: CHECK: expected string not found in input
 // CHECK: call noundef float @llvm.sqrt.f32(float noundef 4.000000e+00
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/call-llvm-intrinsics/call-llvm-intrinsics.ll:1:1: note: scanning from here
; ModuleID = 'call_llvm_intrinsics.61da391d-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/call-llvm-intrinsics/call-llvm-intrinsics.ll:33:7: note: possible intended match here
 %2 = call i1 @llvm.expect.i1(i1 %_15.1, i1 false)

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/call-llvm-intrinsics/call-llvm-intrinsics.ll
Check file: /checkout/src/test/codegen/call-llvm-intrinsics.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'call_llvm_intrinsics.61da391d-cgu.0' 
check:26'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            2: source_filename = "call_llvm_intrinsics.61da391d-cgu.0" 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            4: target triple = "x86_64-unknown-linux-gnu" 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            5:  
check:26'0     ~
            6: %"core::fmt::Arguments<'_>" = type { { ptr, i64 }, { ptr, i64 }, { ptr, i64 } } 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            7: %A = type {} 
check:26'0     ~~~~~~~~~~~~~
            8:  
check:26'0     ~
            9: @alloc17 = private unnamed_addr constant <{ [37 x i8] }> <{ [37 x i8] c"/checkout/library/core/src/fmt/mod.rs" }>, align 1 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           10: @alloc16 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc17, [16 x i8] c"%\00\00\00\00\00\00\00\8B\01\00\008\00\00\00" }>, align 8 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           11: @str.0 = internal constant [28 x i8] c"attempt to add with overflow" 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           12: @alloc11 = private unnamed_addr constant <{ [12 x i8] }> <{ [12 x i8] c"invalid args" }>, align 1 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           13: @alloc12 = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc11, [8 x i8] c"\0C\00\00\00\00\00\00\00" }>, align 8 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           14: @alloc8 = private unnamed_addr constant <{}> zeroinitializer, align 8 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           15: @alloc18 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc17, [16 x i8] c"%\00\00\00\00\00\00\00\8C\01\00\00\0D\00\00\00" }>, align 8 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           16: @alloc5 = private unnamed_addr constant <{ [2 x i8] }> <{ [2 x i8] c"A\0A" }>, align 1 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           17: @alloc6 = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc5, [8 x i8] c"\02\00\00\00\00\00\00\00" }>, align 8 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           18:  
check:26'0     ~
           19: ; core::fmt::Arguments::new_v1 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           20: ; Function Attrs: inlinehint nonlazybind uwtable 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           21: define internal void @_ZN4core3fmt9Arguments6new_v117h1f3b154fe22ead83E(ptr sret(%"core::fmt::Arguments<'_>") %0, ptr align 8 %pieces.0, i64 %pieces.1, ptr align 8 %args.0, i64 %args.1) unnamed_addr #0 { 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           22: start: 
check:26'0     ~~~~~~~
           23:  %_25 = alloca { ptr, i64 }, align 8 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           24:  %_17 = alloca %"core::fmt::Arguments<'_>", align 8 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           25:  %_3 = alloca i8, align 1 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
           26:  %_4 = icmp ult i64 %pieces.1, %args.1 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           27:  br i1 %_4, label %bb1, label %bb2 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           28:  
check:26'0     ~
           29: bb2: ; preds = %start 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~
           30:  %1 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %args.1, i64 1) 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           31:  %_15.0 = extractvalue { i64, i1 } %1, 0 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           32:  %_15.1 = extractvalue { i64, i1 } %1, 1 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           33:  %2 = call i1 @llvm.expect.i1(i1 %_15.1, i1 false) 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:26'1           ?                                             possible intended match
           34:  br i1 %2, label %panic, label %bb4 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           35:  
check:26'0     ~
           36: bb1: ; preds = %start 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~
           37:  store i8 1, ptr %_3, align 1 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           38:  br label %bb3 
check:26'0     ~~~~~~~~~~~~~~~
           39:  
check:26'0     ~
           40: bb3: ; preds = %bb4, %bb1 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
           41:  %3 = load i8, ptr %_3, align 1, !range !2, !noundef !3 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           42:  %4 = trunc i8 %3 to i1 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~
           43:  br i1 %4, label %bb5, label %bb7 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           44:  
check:26'0     ~
           45: bb4: ; preds = %bb2 
check:26'0     ~~~~~~~~~~~~~~~~~~~~
           46:  %_9 = icmp ugt i64 %pieces.1, %_15.0 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           47:  %5 = zext i1 %_9 to i8 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~
           48:  store i8 %5, ptr %_3, align 1 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           49:  br label %bb3 
check:26'0     ~~~~~~~~~~~~~~~
           50:  
check:26'0     ~
           51: panic: ; preds = %bb2 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~
           52: ; call core::panicking::panic 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           53:  call void @_ZN4core9panicking5panic17hac26fc589d2d01b2E(ptr align 1 @str.0, i64 28, ptr align 8 @alloc16) #5 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           54:  unreachable 
check:26'0     ~~~~~~~~~~~~~
           55:  
check:26'0     ~
           56: bb7: ; preds = %bb3 
check:26'0     ~~~~~~~~~~~~~~~~~~~~
           57:  store ptr null, ptr %_25, align 8 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           58:  %6 = getelementptr inbounds %"core::fmt::Arguments<'_>", ptr %0, i32 0, i32 1 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           59:  %7 = getelementptr inbounds { ptr, i64 }, ptr %6, i32 0, i32 0 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           60:  store ptr %pieces.0, ptr %7, align 8 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           61:  %8 = getelementptr inbounds { ptr, i64 }, ptr %6, i32 0, i32 1 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           62:  store i64 %pieces.1, ptr %8, align 8 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           63:  %9 = getelementptr inbounds { ptr, i64 }, ptr %_25, i32 0, i32 0 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           64:  %10 = load ptr, ptr %9, align 8, !align !4, !noundef !3 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           65:  %11 = getelementptr inbounds { ptr, i64 }, ptr %_25, i32 0, i32 1 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           66:  %12 = load i64, ptr %11, align 8 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           67:  %13 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 0 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           68:  store ptr %10, ptr %13, align 8 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           69:  %14 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 1 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           70:  store i64 %12, ptr %14, align 8 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           71:  %15 = getelementptr inbounds %"core::fmt::Arguments<'_>", ptr %0, i32 0, i32 2 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           72:  %16 = getelementptr inbounds { ptr, i64 }, ptr %15, i32 0, i32 0 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           73:  store ptr %args.0, ptr %16, align 8 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           74:  %17 = getelementptr inbounds { ptr, i64 }, ptr %15, i32 0, i32 1 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           75:  store i64 %args.1, ptr %17, align 8 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           76:  ret void 
check:26'0     ~~~~~~~~~~
           77:  
check:26'0     ~
           78: bb5: ; preds = %bb3 
check:26'0     ~~~~~~~~~~~~~~~~~~~~
           79: ; call core::fmt::Arguments::new_v1 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           80:  call void @_ZN4core3fmt9Arguments6new_v117h1f3b154fe22ead83E(ptr sret(%"core::fmt::Arguments<'_>") %_17, ptr align 8 @alloc12, i64 1, ptr align 8 @alloc8, i64 0) 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           81: ; call core::panicking::panic_fmt 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           82:  call void @_ZN4core9panicking9panic_fmt17h3b6b036771e30db0E(ptr %_17, ptr align 8 @alloc18) #5 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           83:  unreachable 
check:26'0     ~~~~~~~~~~~~~
           84: } 
check:26'0     ~~
           85:  
check:26'0     ~
           86: ; core::ptr::drop_in_place<call_llvm_intrinsics::A> 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           87: ; Function Attrs: nonlazybind uwtable 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           88: define void @"_ZN4core3ptr44drop_in_place$LT$call_llvm_intrinsics..A$GT$17h4c61ea66aed3f78fE"(ptr %_1) unnamed_addr #1 { 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           89: start: 
check:26'0     ~~~~~~~
           90: ; call <call_llvm_intrinsics::A as core::ops::drop::Drop>::drop 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           91:  call void @"_ZN65_$LT$call_llvm_intrinsics..A$u20$as$u20$core..ops..drop..Drop$GT$4drop17h2573a2f0e9f43b1eE"(ptr align 1 %_1) 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           92:  ret void 
check:26'0     ~~~~~~~~~~
           93: } 
check:26'0     ~~
           94:  
check:26'0     ~
           95: ; <call_llvm_intrinsics::A as core::ops::drop::Drop>::drop 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           96: ; Function Attrs: nonlazybind uwtable 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           97: define void @"_ZN65_$LT$call_llvm_intrinsics..A$u20$as$u20$core..ops..drop..Drop$GT$4drop17h2573a2f0e9f43b1eE"(ptr align 1 %self) unnamed_addr #1 { 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           98: start: 
check:26'0     ~~~~~~~
           99:  %_3 = alloca %"core::fmt::Arguments<'_>", align 8 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          100: ; call core::fmt::Arguments::new_v1 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          101:  call void @_ZN4core3fmt9Arguments6new_v117h1f3b154fe22ead83E(ptr sret(%"core::fmt::Arguments<'_>") %_3, ptr align 8 @alloc6, i64 1, ptr align 8 @alloc8, i64 0) 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          102: ; call std::io::stdio::_print 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          103:  call void @_ZN3std2io5stdio6_print17h3f412cef86c9c062E(ptr %_3) 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          104:  ret void 
check:26'0     ~~~~~~~~~~
          105: } 
check:26'0     ~~
          106:  
check:26'0     ~
          107: ; call_llvm_intrinsics::do_call 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          108: ; Function Attrs: nonlazybind uwtable 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          109: define void @_ZN20call_llvm_intrinsics7do_call17he65a95b0fca71fc3E() unnamed_addr #1 { 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          110: start: 
check:26'0     ~~~~~~~
          111:  %_a = alloca %A, align 1 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
          112:  %_2 = call float @llvm.sqrt.f32(float 4.000000e+00) #6 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          113: ; call core::ptr::drop_in_place<call_llvm_intrinsics::A> 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          114:  call void @"_ZN4core3ptr44drop_in_place$LT$call_llvm_intrinsics..A$GT$17h4c61ea66aed3f78fE"(ptr %_a) 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          115:  ret void 
check:26'0     ~~~~~~~~~~
          116: } 
check:26'0     ~~
          117:  
check:26'0     ~
          118: ; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          119: declare { i64, i1 } @llvm.uadd.with.overflow.i64(i64, i64) #2 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          120:  
check:26'0     ~
          121: ; Function Attrs: nocallback nofree nosync nounwind readnone willreturn 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          122: declare i1 @llvm.expect.i1(i1, i1) #3 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          123:  
check:26'0     ~
          124: ; core::panicking::panic 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~
