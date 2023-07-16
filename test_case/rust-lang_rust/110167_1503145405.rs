plain
failures:

---- [codegen] tests/codegen/issues/issue-101814.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issues/issue-101814/issue-101814.ll" "/checkout/tests/codegen/issues/issue-101814.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/tests/codegen/issues/issue-101814.rs:11:12: error: CHECK: expected string not found in input
 // CHECK: [[R:%.+]] = add i32 [[L1]], [[L2]]
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issues/issue-101814/issue-101814.ll:59:19: note: scanning from here
 %other = load i32, ptr %iter.sroa.5.0.ptr31, align 4, !noundef !12
                  ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issues/issue-101814/issue-101814.ll:59:19: note: with "L1" equal to "%other\\.peel"
 %other = load i32, ptr %iter.sroa.5.0.ptr31, align 4, !noundef !12
                  ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issues/issue-101814/issue-101814.ll:59:19: note: with "L2" equal to "%other"
 %other = load i32, ptr %iter.sroa.5.0.ptr31, align 4, !noundef !12
                  ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issues/issue-101814/issue-101814.ll:60:3: note: possible intended match here
 %_15.0 = add i32 %other, %other.peel

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issues/issue-101814/issue-101814.ll
Check file: /checkout/tests/codegen/issues/issue-101814.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'issue_101814.5807ccee-cgu.0' 
            2: source_filename = "issue_101814.5807ccee-cgu.0" 
            3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
            4: target triple = "x86_64-unknown-linux-gnu" 
            5:  
            6: @alloc_a7eadb19214e4a0061b3aa6a3e0d6a15 = private unnamed_addr constant <{ [40 x i8] }> <{ [40 x i8] c"/checkout/library/core/src/slice/iter.rs" }>, align 1 
            7: @alloc_bbc42f5b608722ecf25803d2bc65b511 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_a7eadb19214e4a0061b3aa6a3e0d6a15, [16 x i8] c"(\00\00\00\00\00\00\00\81\00\00\00\01\00\00\00" }>, align 8 
            8:  
            9: ; Function Attrs: nounwind nonlazybind uwtable 
           10: define noundef i32 @test(ptr noalias nocapture noundef readonly dereferenceable(40) %a) unnamed_addr #0 personality ptr @rust_eh_personality { 
           11: bb15.i.i.peel: 
           12:  %.ptr14.peel = getelementptr inbounds i32, ptr %a, i64 8 
           13:  %_77.i.i.peel = ptrtoint ptr %.ptr14.peel to i64 
           14:  %_26.i.i.peel = and i64 %_77.i.i.peel, 3 
           15:  %_27.i3.i.peel = icmp eq i64 %_26.i.i.peel, 0 
           16:  br i1 %_27.i3.i.peel, label %bb3.peel, label %panic.i4.i, !prof !2 
           17:  
           18: bb3.peel: ; preds = %bb15.i.i.peel 
           19:  %iter.sroa.5.0.ptr31 = getelementptr inbounds i32, ptr %a, i64 9 
           20:  %a26 = ptrtoint ptr %a to i64 
           21:  %0 = and i64 %a26, 3 
           22:  %_22.i.i = icmp eq i64 %0, 0 
           23:  br i1 %_22.i.i, label %bb3, label %panic.i.i, !prof !3 
           24:  
           25: panic.i.i: ; preds = %bb3.peel 
           26:  %_27.i.i.le = ptrtoint ptr %iter.sroa.5.0.ptr31 to i64 
           27: ; invoke core::panicking::panic_misaligned_pointer_dereference 
           28:  invoke void @_ZN4core9panicking36panic_misaligned_pointer_dereference17hfe2b5553f59c5bedE(i64 noundef 4, i64 noundef %_27.i.i.le, ptr noalias noundef nonnull readonly align 8 dereferenceable(24) @alloc_bbc42f5b608722ecf25803d2bc65b511) #4 
           29:  to label %unreachable.i.i unwind label %terminate.i.i, !noalias !4 
           30:  
           31: terminate.i.i: ; preds = %panic.i.i 
           32:  %1 = landingpad { ptr, i32 } 
           33:  cleanup 
           34: ; call core::panicking::panic_cannot_unwind 
           35:  tail call void @_ZN4core9panicking19panic_cannot_unwind17h2fde18bc291a9074E() #5, !noalias !4 
           36:  unreachable 
           37:  
           38: unreachable.i.i: ; preds = %panic.i.i 
           39:  unreachable 
           40:  
           41: panic.i4.i: ; preds = %bb15.i.i.peel 
           42:  %_77.i.i.le = ptrtoint ptr %.ptr14.peel to i64 
           43: ; invoke core::panicking::panic_misaligned_pointer_dereference 
           44:  invoke void @_ZN4core9panicking36panic_misaligned_pointer_dereference17hfe2b5553f59c5bedE(i64 noundef 4, i64 noundef %_77.i.i.le, ptr noalias noundef nonnull readonly align 8 dereferenceable(24) @alloc_bbc42f5b608722ecf25803d2bc65b511) #4 
           45:  to label %unreachable.i6.i unwind label %terminate.i5.i, !noalias !9 
           46:  
           47: terminate.i5.i: ; preds = %panic.i4.i 
           48:  %2 = landingpad { ptr, i32 } 
           49:  cleanup 
           50: ; call core::panicking::panic_cannot_unwind 
           51:  tail call void @_ZN4core9panicking19panic_cannot_unwind17h2fde18bc291a9074E() #5, !noalias !9 
           52:  unreachable 
           53:  
           54: unreachable.i6.i: ; preds = %panic.i4.i 
           55:  unreachable 
           56:  
           57: bb3: ; preds = %bb3.peel 
           58:  %other.peel = load i32, ptr %.ptr14.peel, align 4, !noundef !12 
           59:  %other = load i32, ptr %iter.sroa.5.0.ptr31, align 4, !noundef !12 
check:11'0                       X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
check:11'1                                                                          with "L1" equal to "%other\\.peel"
check:11'2                                                                          with "L2" equal to "%other"
           60:  %_15.0 = add i32 %other, %other.peel 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:11'3       ?                                    possible intended match
           61:  ret i32 %_15.0 
check:11'0     ~~~~~~~~~~~~~~~~
           62: } 
check:11'0     ~~
           63:  
check:11'0     ~
           64: ; Function Attrs: nonlazybind uwtable 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           65: declare noundef i32 @rust_eh_personality(i32 noundef, i32 noundef, i64 noundef, ptr noundef, ptr noundef) unnamed_addr #1 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           66:  
check:11'0     ~
           67: ; core::panicking::panic_misaligned_pointer_dereference 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           68: ; Function Attrs: cold noinline noreturn nonlazybind uwtable 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           69: declare void @_ZN4core9panicking36panic_misaligned_pointer_dereference17hfe2b5553f59c5bedE(i64 noundef, i64 noundef, ptr noalias noundef readonly align 8 dereferenceable(24)) unnamed_addr #2 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           70:  
check:11'0     ~
           71: ; core::panicking::panic_cannot_unwind 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           72: ; Function Attrs: cold noinline noreturn nounwind nonlazybind uwtable 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           73: declare void @_ZN4core9panicking19panic_cannot_unwind17h2fde18bc291a9074E() unnamed_addr #3 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           74:  
check:11'0     ~
           75: attributes #0 = { nounwind nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" } 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           76: attributes #1 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" } 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           77: attributes #2 = { cold noinline noreturn nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" } 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           78: attributes #3 = { cold noinline noreturn nounwind nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" } 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           79: attributes #4 = { noreturn } 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           80: attributes #5 = { noinline noreturn nounwind } 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           81:  
check:11'0     ~
           82: !llvm.module.flags = !{!0, !1} 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           83:  
check:11'0     ~
           84: !0 = !{i32 8, !"PIC Level", i32 2} 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           85: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           86: !2 = !{!"branch_weights", i32 2000, i32 1} 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           87: !3 = !{!"branch_weights", i32 1999, i32 1} 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           88: !4 = !{!5, !7} 
check:11'0     ~~~~~~~~~~~~~~~
           89: !5 = distinct !{!5, !6, !"_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h1363c1b509f5468dE: %self"} 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           90: !6 = distinct !{!6, !"_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h1363c1b509f5468dE"} 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           91: !7 = distinct !{!7, !8, !"_ZN100_$LT$core..iter..adapters..skip..Skip$LT$I$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hfb858162eea9c05eE: %self"} 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           92: !8 = distinct !{!8, !"_ZN100_$LT$core..iter..adapters..skip..Skip$LT$I$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hfb858162eea9c05eE"} 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           93: !9 = !{!10, !7} 
check:11'0     ~~~~~~~~~~~~~~~~
           94: !10 = distinct !{!10, !11, !"_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$3nth17h77ab52d326373d3fE: %self"} 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           95: !11 = distinct !{!11, !"_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$3nth17h77ab52d326373d3fE"} 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           96: !12 = !{} 
check:11'0     ~~~~~~~~~~
------------------------------------------


---- [codegen] tests/codegen/issues/issue-101082.rs stdout ----
---- [codegen] tests/codegen/issues/issue-101082.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issues/issue-101082/issue-101082.ll" "/checkout/tests/codegen/issues/issue-101082.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/tests/codegen/issues/issue-101082.rs:9:12: error: CHECK: expected string not found in input
 // CHECK: ret i64 165
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issues/issue-101082/issue-101082.ll:9:26: note: scanning from here
define noundef i64 @test() unnamed_addr #0 personality ptr @rust_eh_personality {
                         ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issues/issue-101082/issue-101082.ll:51:2: note: possible intended match here
 ret i64 %15

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issues/issue-101082/issue-101082.ll
Check file: /checkout/tests/codegen/issues/issue-101082.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
           1: ; ModuleID = 'issue_101082.6ae14e6e-cgu.0' 
           2: source_filename = "issue_101082.6ae14e6e-cgu.0" 
           3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
           4: target triple = "x86_64-unknown-linux-gnu" 
           5:  
           6: %"core::array::iter::IntoIter<usize, 6>" = type { [6 x i64], { i64, i64 } } 
           7:  
           8: ; Function Attrs: nonlazybind uwtable 
           9: define noundef i64 @test() unnamed_addr #0 personality ptr @rust_eh_personality { 
check:9'0                              X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
          10: start: 
check:9'0     ~~~~~~~
          11:  %iter = alloca %"core::array::iter::IntoIter<usize, 6>", align 8 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          12:  call void @llvm.lifetime.start.p0(i64 64, ptr nonnull %iter) 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          13:  store i64 23, ptr %iter, align 8 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          14:  %_2.sroa.0.sroa.4.0.iter.sroa_idx = getelementptr inbounds i8, ptr %iter, i64 8 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          15:  store i64 16, ptr %_2.sroa.0.sroa.4.0.iter.sroa_idx, align 8 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          16:  %_2.sroa.0.sroa.5.0.iter.sroa_idx = getelementptr inbounds i8, ptr %iter, i64 16 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          17:  store i64 54, ptr %_2.sroa.0.sroa.5.0.iter.sroa_idx, align 8 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          18:  %_2.sroa.0.sroa.6.0.iter.sroa_idx = getelementptr inbounds i8, ptr %iter, i64 24 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          19:  store i64 3, ptr %_2.sroa.0.sroa.6.0.iter.sroa_idx, align 8 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          20:  %_2.sroa.0.sroa.7.0.iter.sroa_idx = getelementptr inbounds i8, ptr %iter, i64 32 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          21:  store i64 60, ptr %_2.sroa.0.sroa.7.0.iter.sroa_idx, align 8 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          22:  %_2.sroa.0.sroa.8.0.iter.sroa_idx = getelementptr inbounds i8, ptr %iter, i64 40 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          23:  store i64 9, ptr %_2.sroa.0.sroa.8.0.iter.sroa_idx, align 8 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          24:  %_2.sroa.4.0.iter.sroa_idx = getelementptr inbounds i8, ptr %iter, i64 48 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          25:  store i64 0, ptr %_2.sroa.4.0.iter.sroa_idx, align 8 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          26:  %_2.sroa.5.0.iter.sroa_idx = getelementptr inbounds i8, ptr %iter, i64 56 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          27:  store i64 6, ptr %_2.sroa.5.0.iter.sroa_idx, align 8 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          28:  store i64 1, ptr %_2.sroa.4.0.iter.sroa_idx, align 8, !alias.scope !2 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          29:  %0 = load i64, ptr %iter, align 8, !alias.scope !7, !noundef !12 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          30:  store i64 2, ptr %_2.sroa.4.0.iter.sroa_idx, align 8, !alias.scope !2 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          31:  %1 = getelementptr inbounds i64, ptr %iter, i64 1 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          32:  %2 = load i64, ptr %1, align 8, !alias.scope !7, !noundef !12 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          33:  store i64 3, ptr %_2.sroa.4.0.iter.sroa_idx, align 8, !alias.scope !2 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          34:  %3 = getelementptr inbounds i64, ptr %iter, i64 2 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          35:  %4 = load i64, ptr %3, align 8, !alias.scope !7, !noundef !12 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          36:  store i64 4, ptr %_2.sroa.4.0.iter.sroa_idx, align 8, !alias.scope !2 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          37:  %5 = getelementptr inbounds i64, ptr %iter, i64 3 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          38:  %6 = load i64, ptr %5, align 8, !alias.scope !7, !noundef !12 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          39:  store i64 5, ptr %_2.sroa.4.0.iter.sroa_idx, align 8, !alias.scope !2 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          40:  %7 = getelementptr inbounds i64, ptr %iter, i64 4 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          41:  %8 = load i64, ptr %7, align 8, !alias.scope !7, !noundef !12 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          42:  %9 = add i64 %2, %0 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~
          43:  %10 = add i64 %4, %9 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~
          44:  %11 = add i64 %6, %10 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~
          45:  %12 = add i64 %8, %11 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~
          46:  store i64 6, ptr %_2.sroa.4.0.iter.sroa_idx, align 8, !alias.scope !2 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          47:  %13 = getelementptr inbounds i64, ptr %iter, i64 5 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          48:  %14 = load i64, ptr %13, align 8, !alias.scope !7, !noundef !12 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          49:  %15 = add i64 %14, %12 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~
          50:  call void @llvm.lifetime.end.p0(i64 64, ptr nonnull %iter) 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          51:  ret i64 %15 
check:9'0     ~~~~~~~~~~~~~
check:9'1      ?            possible intended match
          52: } 
check:9'0     ~~
          53:  
check:9'0     ~
          54: ; Function Attrs: nonlazybind uwtable 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          55: declare noundef i32 @rust_eh_personality(i32 noundef, i32 noundef, i64 noundef, ptr noundef, ptr noundef) unnamed_addr #0 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          56:  
check:9'0     ~
          57: ; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          58: declare void @llvm.lifetime.start.p0(i64 immarg, ptr nocapture) #1 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          59:  
check:9'0     ~
          60: ; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          61: declare void @llvm.lifetime.end.p0(i64 immarg, ptr nocapture) #1 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          62:  
check:9'0     ~
          63: attributes #0 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" } 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          64: attributes #1 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) } 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          65:  
check:9'0     ~
          66: !llvm.module.flags = !{!0, !1} 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          67:  
check:9'0     ~
          68: !0 = !{i32 8, !"PIC Level", i32 2} 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          69: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          70: !2 = !{!3, !5} 
check:9'0     ~~~~~~~~~~~~~~~
          71: !3 = distinct !{!3, !4, !"_ZN93_$LT$core..ops..index_range..IndexRange$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hcd06cf948aedbc02E: argument 0"} 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          72: !4 = distinct !{!4, !"_ZN93_$LT$core..ops..index_range..IndexRange$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hcd06cf948aedbc02E"} 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          73: !5 = distinct !{!5, !6, !"_ZN99_$LT$core..array..iter..IntoIter$LT$T$C$_$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h8c4aa3b080570dfdE: %self"} 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          74: !6 = distinct !{!6, !"_ZN99_$LT$core..array..iter..IntoIter$LT$T$C$_$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h8c4aa3b080570dfdE"} 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          75: !7 = !{!8, !10, !5} 
check:9'0     ~~~~~~~~~~~~~~~~~~~~
          76: !8 = distinct !{!8, !9, !"_ZN99_$LT$core..array..iter..IntoIter$LT$T$C$_$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next28_$u7b$$u7b$closure$u7d$$u7d$17h7d659d5703b2e0d6E: argument 0"} 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          77: !9 = distinct !{!9, !"_ZN99_$LT$core..array..iter..IntoIter$LT$T$C$_$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next28_$u7b$$u7b$closure$u7d$$u7d$17h7d659d5703b2e0d6E"} 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          78: !10 = distinct !{!10, !11, !"_ZN4core6option15Option$LT$T$GT$3map17h49e298c18e57e707E: %f"} 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          79: !11 = distinct !{!11, !"_ZN4core6option15Option$LT$T$GT$3map17h49e298c18e57e707E"} 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          80: !12 = !{} 
check:9'0     ~~~~~~~~~~
------------------------------------------



