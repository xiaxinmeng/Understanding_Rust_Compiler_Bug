plain
failures:

---- [codegen] src/test/codegen/enum-match.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/enum-match/enum-match.ll" "/checkout/src/test/codegen/enum-match.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/enum-match.rs:18:16: error: CHECK-NEXT: expected string not found in input
// CHECK-NEXT: %.0 = select i1 %1, i8 13, i8 %2
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/enum-match/enum-match.ll:10:19: note: scanning from here
 %2 = and i8 %0, 1
                  ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/enum-match/enum-match.ll:11:13: note: possible intended match here
 %common.ret.op = select i1 %1, i8 13, i8 %2

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/enum-match/enum-match.ll
Check file: /checkout/src/test/codegen/enum-match.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
           1: ; ModuleID = 'enum_match.fe491799-cgu.0' 
           2: source_filename = "enum_match.fe491799-cgu.0" 
           3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
           4: target triple = "x86_64-unknown-linux-gnu" 
           5:  
           6: ; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind readnone uwtable willreturn 
           7: define i8 @match0(i8 noundef %0) unnamed_addr #0 { 
           8: start: 
           9:  %1 = icmp eq i8 %0, 2 
          10:  %2 = and i8 %0, 1 
next:18'0                       X error: no match found
          11:  %common.ret.op = select i1 %1, i8 13, i8 %2 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
next:18'1                 ?                                 possible intended match
          12:  ret i8 %common.ret.op 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~
          13: } 
next:18'0     ~~
          14:  
next:18'0     ~
          15: ; Function Attrs: mustprogress nofree nosync nounwind nonlazybind readnone uwtable willreturn 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          16: define i8 @match1(i8 noundef %0) unnamed_addr #1 { 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          17: start: 
next:18'0     ~~~~~~~
          18:  %1 = call i8 @llvm.usub.sat.i8(i8 %0, i8 1) 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          19:  switch i8 %1, label %bb2 [ 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          20:  i8 0, label %bb3 
next:18'0     ~~~~~~~~~~~~~~~~~~
          21:  i8 1, label %common.ret 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~
          22:  i8 2, label %bb1 
next:18'0     ~~~~~~~~~~~~~~~~~~
          23:  ] 
next:18'0     ~~~
          24:  
next:18'0     ~
          25: bb2: ; preds = %start 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~
          26:  unreachable 
next:18'0     ~~~~~~~~~~~~~
          27:  
next:18'0     ~
          28: common.ret: ; preds = %start, %bb1, %bb3 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          29:  %common.ret.op = phi i8 [ %2, %bb3 ], [ 100, %bb1 ], [ 13, %start ] 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          30:  ret i8 %common.ret.op 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~
          31:  
next:18'0     ~
          32: bb3: ; preds = %start 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~
          33:  %2 = and i8 %0, 1 
next:18'0     ~~~~~~~~~~~~~~~~~~~
          34:  br label %common.ret 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~
          35:  
next:18'0     ~
          36: bb1: ; preds = %start 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~
          37:  br label %common.ret 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~
          38: } 
next:18'0     ~~
          39:  
next:18'0     ~
          40: ; Function Attrs: mustprogress nofree nosync nounwind nonlazybind uwtable willreturn 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          41: define i8 @match2(i8 %0) unnamed_addr #2 { 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          42: start: 
next:18'0     ~~~~~~~
          43:  %1 = add i8 %0, 2 
next:18'0     ~~~~~~~~~~~~~~~~~~~
          44:  %2 = zext i8 %1 to i64 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~
          45:  %3 = icmp ult i8 %1, 4 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~
          46:  %4 = add nuw nsw i64 %2, 1 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          47:  %_2 = select i1 %3, i64 %4, i64 0 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          48:  switch i64 %_2, label %bb2 [ 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          49:  i64 0, label %bb3 
next:18'0     ~~~~~~~~~~~~~~~~~~~
          50:  i64 1, label %common.ret 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
          51:  i64 2, label %bb5 
next:18'0     ~~~~~~~~~~~~~~~~~~~
          52:  i64 3, label %bb6 
next:18'0     ~~~~~~~~~~~~~~~~~~~
          53:  i64 4, label %bb1 
next:18'0     ~~~~~~~~~~~~~~~~~~~
          54:  ] 
next:18'0     ~~~
          55:  
next:18'0     ~
          56: bb2: ; preds = %start 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~
          57:  unreachable 
next:18'0     ~~~~~~~~~~~~~
          58:  
next:18'0     ~
          59: common.ret: ; preds = %start, %bb1, %bb6, %bb5, %bb3 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          60:  %common.ret.op = phi i8 [ %0, %bb3 ], [ 100, %bb5 ], [ -56, %bb6 ], [ -6, %bb1 ], [ 13, %start ] 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          61:  ret i8 %common.ret.op 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~
          62:  
next:18'0     ~
          63: bb3: ; preds = %start 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~
          64:  %_6 = icmp ult i8 %0, -2 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
          65:  call void @llvm.assume(i1 %_6) 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          66:  %_7 = icmp ugt i8 %0, 1 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~
          67:  call void @llvm.assume(i1 %_7) 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          68:  br label %common.ret 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~
          69:  
next:18'0     ~
          70: bb5: ; preds = %start 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~
          71:  br label %common.ret 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~
          72:  
next:18'0     ~
          73: bb6: ; preds = %start 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~
          74:  br label %common.ret 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~
          75:  
next:18'0     ~
          76: bb1: ; preds = %start 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~
          77:  br label %common.ret 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~
          78: } 
next:18'0     ~~
          79:  
next:18'0     ~
          80: ; Function Attrs: inaccessiblememonly mustprogress nofree nosync nounwind willreturn 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          81: declare void @llvm.assume(i1 noundef) #3 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          82:  
next:18'0     ~
          83: ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          84: declare i8 @llvm.usub.sat.i8(i8, i8) #4 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          85:  
next:18'0     ~
          86: attributes #0 = { mustprogress nofree norecurse nosync nounwind nonlazybind readnone uwtable willreturn "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          87: attributes #1 = { mustprogress nofree nosync nounwind nonlazybind readnone uwtable willreturn "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          88: attributes #2 = { mustprogress nofree nosync nounwind nonlazybind uwtable willreturn "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          89: attributes #3 = { inaccessiblememonly mustprogress nofree nosync nounwind willreturn } 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          90: attributes #4 = { nofree nosync nounwind readnone speculatable willreturn } 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          91:  
next:18'0     ~
          92: !llvm.module.flags = !{!0, !1} 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          93:  
next:18'0     ~
          94: !0 = !{i32 7, !"PIC Level", i32 2} 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          95: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
next:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
------------------------------------------


---- [codegen] src/test/codegen/match-optimized.rs stdout ----
---- [codegen] src/test/codegen/match-optimized.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/match-optimized/match-optimized.ll" "/checkout/src/test/codegen/match-optimized.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/match-optimized.rs:24:16: error: CHECK-NEXT: expected string not found in input
// CHECK-NEXT: br label %[[EXIT:[a-zA-Z0-9_]+]]
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/match-optimized/match-optimized.ll:24:29: note: scanning from here
 store i8 0, i8* %1, align 1
                            ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/match-optimized/match-optimized.ll:28:8: note: possible intended match here
bb4: ; preds = %start

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/match-optimized/match-optimized.ll
Check file: /checkout/src/test/codegen/match-optimized.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
           1: ; ModuleID = 'match_optimized.9fa91b20-cgu.0' 
           2: source_filename = "match_optimized.9fa91b20-cgu.0" 
           3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
           4: target triple = "x86_64-unknown-linux-gnu" 
           5:  
           6: ; Function Attrs: nonlazybind uwtable 
           7: define i8 @exhaustive_match(i8 noundef %0) unnamed_addr #0 { 
           8: start: 
           9:  %1 = alloca i8, align 1 
          10:  %e = alloca i8, align 1 
          11:  store i8 %0, i8* %e, align 1 
          12:  %2 = load i8, i8* %e, align 1, !range !2, !noundef !3 
          13:  %_2 = zext i8 %2 to i64 
          14:  switch i64 %_2, label %bb2 [ 
          15:  i64 0, label %bb3 
          16:  i64 1, label %bb4 
          17:  i64 2, label %bb1 
          18:  ] 
          19:  
          20: bb2: ; preds = %start 
          21:  unreachable 
          22:  
          23: bb3: ; preds = %start 
          24:  store i8 0, i8* %1, align 1 
next:24'0                                 X error: no match found
          25:  %3 = load i8, i8* %1, align 1 
next:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          26:  ret i8 %3 
next:24'0     ~~~~~~~~~~~
          27:  
next:24'0     ~
          28: bb4: ; preds = %start 
next:24'0     ~~~~~~~~~~~~~~~~~~~~~~
next:24'1            ?               possible intended match
          29:  store i8 1, i8* %1, align 1 
next:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          30:  %4 = load i8, i8* %1, align 1 
next:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          31:  ret i8 %4 
next:24'0     ~~~~~~~~~~~
          32:  
next:24'0     ~
          33: bb1: ; preds = %start 
next:24'0     ~~~~~~~~~~~~~~~~~~~~~~
          34:  store i8 2, i8* %1, align 1 
next:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          35:  %5 = load i8, i8* %1, align 1 
next:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          36:  ret i8 %5 
next:24'0     ~~~~~~~~~~~
          37: } 
next:24'0     ~~
          38:  
next:24'0     ~
          39: ; Function Attrs: nonlazybind uwtable 
next:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          40: define i8 @exhaustive_match_2(i16 noundef %0) unnamed_addr #0 { 
next:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          41: start: 
          42:  %1 = alloca i8, align 1 
          43:  %e = alloca i16, align 2 
          44:  store i16 %0, i16* %e, align 2 
          45:  %_2 = load i16, i16* %e, align 2, !range !4, !noundef !3 
          46:  switch i16 %_2, label %bb2 [ 
          47:  i16 13, label %bb3 
          48:  i16 42, label %bb1 
          49:  ] 
          50:  
          51: bb2: ; preds = %start 
          52:  unreachable 
          53:  
          54: bb3: ; preds = %start 
          55:  store i8 0, i8* %1, align 1 
          56:  %2 = load i8, i8* %1, align 1 
          57:  ret i8 %2 
          58:  
          59: bb1: ; preds = %start 
          60:  store i8 1, i8* %1, align 1 
          61:  %3 = load i8, i8* %1, align 1 
          62:  ret i8 %3 
          63: } 
          64:  
          65: attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
          66:  
          67: !llvm.module.flags = !{!0, !1} 
          68:  
          69: !0 = !{i32 7, !"PIC Level", i32 2} 
          70: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
          71: !2 = !{i8 0, i8 3} 
          72: !3 = !{} 
          73: !4 = !{i16 13, i16 43} 
------------------------------------------


---- [codegen] src/test/codegen/repeat-trusted-len.rs stdout ----
---- [codegen] src/test/codegen/repeat-trusted-len.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repeat-trusted-len/repeat-trusted-len.ll" "/checkout/src/test/codegen/repeat-trusted-len.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/repeat-trusted-len.rs:11:11: error: CHECK: expected string not found in input
// CHECK: call void @llvm.memset.{{.+}}({{i8\*|ptr}} {{.*}}align 1{{.*}} %{{[0-9]+}}, i8 42, i{{[0-9]+}} 100000, i1 false)
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repeat-trusted-len/repeat-trusted-len.ll:11:33: note: scanning from here
define void @repeat_take_collect(%"alloc::vec::Vec<u8>"* noalias nocapture noundef sret(%"alloc::vec::Vec<u8>") dereferenceable(24) %0) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
/checkout/src/test/codegen/repeat-trusted-len.rs:18:11: error: CHECK: expected string not found in input
/checkout/src/test/codegen/repeat-trusted-len.rs:18:11: error: CHECK: expected string not found in input
// CHECK: call void @llvm.memset.{{.+}}({{i8\*|ptr}} {{.*}}align 1{{.*}} %{{[0-9]+}}, i8 13, i{{[0-9]+}} 12345, i1 false)
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repeat-trusted-len/repeat-trusted-len.ll:62:38: note: scanning from here
define void @repeat_with_take_collect(%"alloc::vec::Vec<u8>"* noalias nocapture noundef sret(%"alloc::vec::Vec<u8>") dereferenceable(24) %0) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repeat-trusted-len/repeat-trusted-len.ll
Check file: /checkout/src/test/codegen/repeat-trusted-len.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
          1: ; ModuleID = 'repeat_trusted_len.073cd506-cgu.0' 
          2: source_filename = "repeat_trusted_len.073cd506-cgu.0" 
          3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
          4: target triple = "x86_64-unknown-linux-gnu" 
          5:  
          6: %"alloc::vec::Vec<u8>" = type { { i64, i8* }, i64 } 
          7: %"unwind::libunwind::_Unwind_Exception" = type { i64, void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [2 x i64] } 
          8: %"unwind::libunwind::_Unwind_Context" = type { [0 x i8] } 
          9:  
         10: ; Function Attrs: nonlazybind uwtable 
         11: define void @repeat_take_collect(%"alloc::vec::Vec<u8>"* noalias nocapture noundef sret(%"alloc::vec::Vec<u8>") dereferenceable(24) %0) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality { 
check:11                                     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
check:11     ~~~~~~~
check:11     ~~~~~~~
         13:  tail call void @llvm.experimental.noalias.scope.decl(metadata !2) 
check:11     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         14:  tail call void @llvm.experimental.noalias.scope.decl(metadata !5) 
check:11     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         15:  tail call void @llvm.experimental.noalias.scope.decl(metadata !8) 
check:11     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         16:  %1 = tail call i8* @__rust_alloc(i64 100000, i64 1) #4, !noalias !11 
check:11     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         17:  %2 = icmp eq i8* %1, null 
check:11     ~~~~~~~~~~~~~~~~~~~~~~~~~~~
         18:  %3 = insertvalue { i8*, i64 } undef, i8* %1, 0 
check:11     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         19:  %4 = insertvalue { i8*, i64 } %3, i64 100000, 1 
check:11     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         20:  %common.ret.op.i.i2.i.i.i = select i1 %2, { i8*, i64 } { i8* null, i64 undef }, { i8*, i64 } %4 
check:11     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         21:  %.fca.0.extract11.i.i.i.i = extractvalue { i8*, i64 } %common.ret.op.i.i2.i.i.i, 0 
check:11     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         22:  %5 = icmp eq i8* %.fca.0.extract11.i.i.i.i, null 
check:11     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         23:  br i1 %5, label %bb19.i.i.i.i, label %"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$11allocate_in17hd17a9990d563c1e9E.exit.i.i.i" 
         24:  
check:11     ~
check:11     ~
         25: bb19.i.i.i.i: ; preds = %start 
         26: ; call alloc::alloc::handle_alloc_error 
check:11     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:11     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         27:  tail call void @_ZN5alloc5alloc18handle_alloc_error17h8b4db88ca5e914afE(i64 100000, i64 noundef 1) #5, !noalias !11 
         28:  unreachable 
check:11     ~~~~~~~~~~~~~
         29:  
check:11     ~
check:11     ~
         30: "_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$11allocate_in17hd17a9990d563c1e9E.exit.i.i.i": ; preds = %start 
check:11     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         31:  %6 = getelementptr inbounds %"alloc::vec::Vec<u8>", %"alloc::vec::Vec<u8>"* %0, i64 0, i32 0, i32 0 
check:11     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         32:  store i64 100000, i64* %6, align 8, !alias.scope !11 
check:11     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         33:  %7 = getelementptr inbounds %"alloc::vec::Vec<u8>", %"alloc::vec::Vec<u8>"* %0, i64 0, i32 0, i32 1 
check:11     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         34:  store i8* %.fca.0.extract11.i.i.i.i, i8** %7, align 8, !alias.scope !11 
check:11     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         35:  tail call void @llvm.experimental.noalias.scope.decl(metadata !12) 
check:11     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         36:  tail call void @llvm.experimental.noalias.scope.decl(metadata !15) 
check:11     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         37:  store i8 42, i8* %.fca.0.extract11.i.i.i.i, align 1, !noalias !18 
check:11     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         38:  br label %bb6.i.i.i.i.i.i.i 
         39:  
check:11     ~
check:11     ~
         40: bb6.i.i.i.i.i.i.i: ; preds = %bb6.i.i.i.i.i.i.i, %"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$11allocate_in17hd17a9990d563c1e9E.exit.i.i.i" 
check:11     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         41:  %common.ret.op.i.i26.i.i.i.i.i.i = phi { i64, i64 } [ %common.ret.op.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i ], [ { i64 0, i64 99998 }, %"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$11allocate_in17hd17a9990d563c1e9E.exit.i.i.i" ] 
check:11     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         42:  %_20.0.i.i.i25.i.i.i.i.i.i = phi i64 [ %_20.0.i.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i ], [ 1, %"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$11allocate_in17hd17a9990d563c1e9E.exit.i.i.i" ] 
check:11     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         43:  %.fca.1.extract12.i.i.i.i.i.i.i = extractvalue { i64, i64 } %common.ret.op.i.i26.i.i.i.i.i.i, 1 
check:11     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         44:  %8 = getelementptr inbounds i8, i8* %.fca.0.extract11.i.i.i.i, i64 %_20.0.i.i.i25.i.i.i.i.i.i 
check:11     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         45:  store i8 42, i8* %8, align 1, !noalias !18 
check:11     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         46:  %_20.0.i.i.i.i.i.i.i.i.i = add i64 %_20.0.i.i.i25.i.i.i.i.i.i, 1 
check:11     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         47:  %.not.i.i.i.i.i.i.i = icmp eq i64 %.fca.1.extract12.i.i.i.i.i.i.i, 0 
check:11     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         48:  %9 = add i64 %.fca.1.extract12.i.i.i.i.i.i.i, -1 
check:11     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         49:  %10 = insertvalue { i64, i64 } { i64 0, i64 undef }, i64 %9, 1 
check:11     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         50:  %common.ret.op.i.i.i.i.i.i.i.i = select i1 %.not.i.i.i.i.i.i.i, { i64, i64 } { i64 1, i64 undef }, { i64, i64 } %10 
check:11     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         51:  %.fca.0.extract11.i.i.i.i.i.i.i = extractvalue { i64, i64 } %common.ret.op.i.i.i.i.i.i.i.i, 0 
check:11     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         52:  %switch.i.i.i.i.i.i.i = icmp eq i64 %.fca.0.extract11.i.i.i.i.i.i.i, 0 
check:11     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         53:  br i1 %switch.i.i.i.i.i.i.i, label %bb6.i.i.i.i.i.i.i, label %"_ZN95_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$T$GT$$GT$9from_iter17hcfb65d1aa7ccd8adE.exit" 
         54:  
check:11     ~
check:11     ~
         55: "_ZN95_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$T$GT$$GT$9from_iter17hcfb65d1aa7ccd8adE.exit": ; preds = %bb6.i.i.i.i.i.i.i 
check:11     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         56:  %11 = getelementptr inbounds %"alloc::vec::Vec<u8>", %"alloc::vec::Vec<u8>"* %0, i64 0, i32 1 
check:11     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         57:  store i64 %_20.0.i.i.i.i.i.i.i.i.i, i64* %11, align 8, !alias.scope !28, !noalias !29 
         58:  ret void 
check:11     ~~~~~~~~~~
         59: } 
check:11     ~~
check:11     ~~
         60:  
check:11     ~
         61: ; Function Attrs: nonlazybind uwtable 
check:11     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         62: define void @repeat_with_take_collect(%"alloc::vec::Vec<u8>"* noalias nocapture noundef sret(%"alloc::vec::Vec<u8>") dereferenceable(24) %0) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality { 
check:11     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:18                                          X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
check:18     ~~~~~~~
check:18     ~~~~~~~
         64:  tail call void @llvm.experimental.noalias.scope.decl(metadata !32) 
check:18     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         65:  tail call void @llvm.experimental.noalias.scope.decl(metadata !35) 
check:18     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         66:  tail call void @llvm.experimental.noalias.scope.decl(metadata !38) 
check:18     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         67:  %1 = tail call i8* @__rust_alloc(i64 12345, i64 1) #4, !noalias !41 
check:18     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         68:  %2 = icmp eq i8* %1, null 
check:18     ~~~~~~~~~~~~~~~~~~~~~~~~~~~
         69:  %3 = insertvalue { i8*, i64 } undef, i8* %1, 0 
check:18     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         70:  %4 = insertvalue { i8*, i64 } %3, i64 12345, 1 
check:18     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         71:  %common.ret.op.i.i2.i.i.i = select i1 %2, { i8*, i64 } { i8* null, i64 undef }, { i8*, i64 } %4 
check:18     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         72:  %.fca.0.extract11.i.i.i.i = extractvalue { i8*, i64 } %common.ret.op.i.i2.i.i.i, 0 
check:18     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         73:  %5 = icmp eq i8* %.fca.0.extract11.i.i.i.i, null 
check:18     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         74:  br i1 %5, label %bb19.i.i.i.i, label %"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$11allocate_in17hd17a9990d563c1e9E.exit.i.i.i" 
         75:  
check:18     ~
check:18     ~
         76: bb19.i.i.i.i: ; preds = %start 
