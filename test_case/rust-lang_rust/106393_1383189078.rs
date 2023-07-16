plain
failures:

---- [codegen] checkout/tests/codegen/issue-45964-bounds-check-slice-pos.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45964-bounds-check-slice-pos/issue-45964-bounds-check-slice-pos.ll" "/checkout/tests/codegen/issue-45964-bounds-check-slice-pos.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/tests/codegen/issue-45964-bounds-check-slice-pos.rs:18:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: panic
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45964-bounds-check-slice-pos/issue-45964-bounds-check-slice-pos.ll:16:14: note: found here
; call core::panicking::panic_nounwind
             ^~~~~
/checkout/tests/codegen/issue-45964-bounds-check-slice-pos.rs:34:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: panic
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45964-bounds-check-slice-pos/issue-45964-bounds-check-slice-pos.ll:63:14: note: found here
; call core::panicking::panic_nounwind

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45964-bounds-check-slice-pos/issue-45964-bounds-check-slice-pos.ll
Check file: /checkout/tests/codegen/issue-45964-bounds-check-slice-pos.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
        1: ; ModuleID = 'issue_45964_bounds_check_slice_pos.8364521e-cgu.0' 
        2: source_filename = "issue_45964_bounds_check_slice_pos.8364521e-cgu.0" 
        3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
        4: target triple = "x86_64-unknown-linux-gnu" 
        5:  
        6: @alloc26 = private unnamed_addr constant <{ [71 x i8] }> <{ [71 x i8] c"unsafe precondition(s) violated: ptr::sub_ptr requires `this >= origin`" }>, align 1 
        7:  
        8: ; Function Attrs: nonlazybind uwtable 
        9: define noundef zeroext i1 @test(ptr noalias noundef nonnull readonly align 4 %y.0, i64 %y.1, ptr noalias noundef readonly align 4 dereferenceable(4) %0, ptr noalias nocapture noundef readonly align 4 dereferenceable(4) %z) unnamed_addr #0 personality ptr @rust_eh_personality { 
       10: start: 
       11:  %1 = getelementptr inbounds i32, ptr %y.0, i64 %y.1 
       12:  %_21.not.i.i = icmp slt i64 %y.1, 0 
       13:  br i1 %_21.not.i.i, label %bb7.i.i, label %"_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7sub_ptr17h3d73225820f4ed81E.exit.i" 
       14:  
       15: bb7.i.i: ; preds = %start 
       16: ; call core::panicking::panic_nounwind 
not:18                  !~~~~                      error: no match expected
       17:  tail call void @_ZN4core9panicking14panic_nounwind17h35f45e596c1fef14E(ptr noalias noundef nonnull readonly align 1 @alloc26, i64 71) #4, !noalias !2 
       18:  unreachable 
       19:  
       20: "_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7sub_ptr17h3d73225820f4ed81E.exit.i": ; preds = %start 
       21:  %_10.i13.i = icmp eq i64 %y.1, 0 
       22:  br i1 %_10.i13.i, label %bb9, label %bb7.lr.ph.i 
       23:  
       24: bb7.lr.ph.i: ; preds = %"_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7sub_ptr17h3d73225820f4ed81E.exit.i" 
       25:  %_11.i.i = load i32, ptr %0, align 4, !noalias !6 
       26:  br label %bb7.i 
       27:  
       28: bb7.i: ; preds = %bb10.i, %bb7.lr.ph.i 
       29:  %i.015.i = phi i64 [ 0, %bb7.lr.ph.i ], [ %_28.0.i, %bb10.i ] 
       30:  %self1.i1214.i = phi ptr [ %y.0, %bb7.lr.ph.i ], [ %3, %bb10.i ] 
       31:  tail call void @llvm.experimental.noalias.scope.decl(metadata !10) 
       32:  %_10.i7.i = load i32, ptr %self1.i1214.i, align 4, !alias.scope !10, !noalias !11 
       33:  %2 = icmp eq i32 %_10.i7.i, %_11.i.i 
       34:  br i1 %2, label %bb6, label %bb10.i 
       35:  
       36: bb10.i: ; preds = %bb7.i 
       37:  %3 = getelementptr inbounds i32, ptr %self1.i1214.i, i64 1 
       38:  %_28.0.i = add nuw nsw i64 %i.015.i, 1 
       39:  %_10.i.i = icmp eq ptr %3, %1 
       40:  br i1 %_10.i.i, label %bb9, label %bb7.i 
       41:  
       42: bb6: ; preds = %bb7.i 
       43:  %_24.i = icmp ult i64 %i.015.i, %y.1 
       44:  tail call void @llvm.assume(i1 %_24.i) 
       45:  %4 = getelementptr inbounds [0 x i32], ptr %y.0, i64 0, i64 %i.015.i 
       46:  %_16 = load i32, ptr %4, align 4 
       47:  %_20 = load i32, ptr %z, align 4 
       48:  %5 = icmp eq i32 %_16, %_20 
       49:  br label %bb9 
       50:  
       51: bb9: ; preds = %bb10.i, %"_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7sub_ptr17h3d73225820f4ed81E.exit.i", %bb6 
       52:  %.0 = phi i1 [ %5, %bb6 ], [ false, %"_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7sub_ptr17h3d73225820f4ed81E.exit.i" ], [ false, %bb10.i ] 
       53:  ret i1 %.0 
       54: } 
       55:  
       56: ; Function Attrs: nonlazybind uwtable 
       57: define noundef zeroext i1 @rtest(ptr noalias noundef nonnull readonly align 4 %y.0, i64 %y.1, ptr noalias noundef readonly align 4 dereferenceable(4) %0, ptr noalias nocapture noundef readonly align 4 dereferenceable(4) %z) unnamed_addr #0 personality ptr @rust_eh_personality { 
       58: start: 
       59:  %_21.not.i.i = icmp slt i64 %y.1, 0 
       60:  br i1 %_21.not.i.i, label %bb7.i.i, label %"_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7sub_ptr17h3d73225820f4ed81E.exit.i" 
       61:  
       62: bb7.i.i: ; preds = %start 
       63: ; call core::panicking::panic_nounwind 
not:34                  !~~~~                      error: no match expected
       64:  tail call void @_ZN4core9panicking14panic_nounwind17h35f45e596c1fef14E(ptr noalias noundef nonnull readonly align 1 @alloc26, i64 71) #4, !noalias !12 
       65:  unreachable 
       66:  
       67: "_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7sub_ptr17h3d73225820f4ed81E.exit.i": ; preds = %start 
       68:  %1 = getelementptr inbounds i32, ptr %y.0, i64 %y.1 
       69:  %_11.i.i = load i32, ptr %0, align 4 
       70:  br label %bb5.i 
       71:  
       72: bb5.i: ; preds = %bb7.i, %"_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7sub_ptr17h3d73225820f4ed81E.exit.i" 
       73:  %2 = phi ptr [ %1, %"_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7sub_ptr17h3d73225820f4ed81E.exit.i" ], [ %3, %bb7.i ] 
       74:  %i.0.i = phi i64 [ %y.1, %"_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7sub_ptr17h3d73225820f4ed81E.exit.i" ], [ %_20.0.i, %bb7.i ] 
       75:  %_10.i.i = icmp eq ptr %2, %y.0 
       76:  br i1 %_10.i.i, label %bb9, label %bb7.i 
       77:  
       78: bb7.i: ; preds = %bb5.i 
       79:  %3 = getelementptr inbounds i32, ptr %2, i64 -1 
       80:  %_20.0.i = add i64 %i.0.i, -1 
       81:  %_10.i10.i = load i32, ptr %3, align 4, !alias.scope !16, !noalias !19 
       82:  %4 = icmp eq i32 %_10.i10.i, %_11.i.i 
       83:  br i1 %4, label %bb6, label %bb5.i 
       84:  
       85: bb6: ; preds = %bb7.i 
       86:  %_25.i = icmp ult i64 %_20.0.i, %y.1 
       87:  tail call void @llvm.assume(i1 %_25.i) 
       88:  %5 = getelementptr inbounds [0 x i32], ptr %y.0, i64 0, i64 %_20.0.i 
       89:  %_16 = load i32, ptr %5, align 4 
       90:  %_20 = load i32, ptr %z, align 4 
       91:  %6 = icmp eq i32 %_16, %_20 
       92:  br label %bb9 
       93:  
       94: bb9: ; preds = %bb5.i, %bb6 
       95:  %.0 = phi i1 [ %6, %bb6 ], [ false, %bb5.i ] 
       96:  ret i1 %.0 
       97: } 
       98:  
       99: ; Function Attrs: inaccessiblememonly mustprogress nocallback nofree nosync nounwind willreturn 
      100: declare void @llvm.assume(i1 noundef) #1 
      102: ; core::panicking::panic_nounwind 
      102: ; core::panicking::panic_nounwind 
      103: ; Function Attrs: cold noinline noreturn nounwind nonlazybind uwtable 
      104: declare void @_ZN4core9panicking14panic_nounwind17h35f45e596c1fef14E(ptr noalias noundef nonnull readonly align 1, i64) unnamed_addr #2 
      105:  
      106: ; Function Attrs: nonlazybind uwtable 
      107: declare noundef i32 @rust_eh_personality(i32, i32 noundef, i64, ptr, ptr) unnamed_addr #0 
      108:  
      109: ; Function Attrs: inaccessiblememonly nocallback nofree nosync nounwind willreturn 
      110: declare void @llvm.experimental.noalias.scope.decl(metadata) #3 
      111:  
      112: attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
      113: attributes #1 = { inaccessiblememonly mustprogress nocallback nofree nosync nounwind willreturn } 
      114: attributes #2 = { cold noinline noreturn nounwind nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
      115: attributes #3 = { inaccessiblememonly nocallback nofree nosync nounwind willreturn } 
      116: attributes #4 = { noreturn nounwind } 
      117:  
      118: !llvm.module.flags = !{!0, !1} 
      119:  
      120: !0 = !{i32 7, !"PIC Level", i32 2} 
      121: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
      122: !2 = !{!3, !5} 
      123: !3 = distinct !{!3, !4, !"_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8position17hbcba0c4c6a0b1514E: %self"} 
      124: !4 = distinct !{!4, !"_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8position17hbcba0c4c6a0b1514E"} 
      125: !5 = distinct !{!5, !4, !"_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$8position17hbcba0c4c6a0b1514E: argument 1"} 
      126: !6 = !{!7, !9, !3, !5} 
      127: !7 = distinct !{!7, !8, !"_ZN34issue_45964_bounds_check_slice_pos4test28_$u7b$$u7b$closure$u7d$$u7d$17h0b46fff7271f21c7E: %_1"} 
      128: !8 = distinct !{!8, !"_ZN34issue_45964_bounds_check_slice_pos4test28_$u7b$$u7b$closure$u7d$$u7d$17h0b46fff7271f21c7E"} 
      129: !9 = distinct !{!9, !8, !"_ZN34issue_45964_bounds_check_slice_pos4test28_$u7b$$u7b$closure$u7d$$u7d$17h0b46fff7271f21c7E: argument 1"} 
      130: !10 = !{!9} 
      131: !11 = !{!7, !3, !5} 
      132: !12 = !{!13, !15} 
      133: !13 = distinct !{!13, !14, !"_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$9rposition17h05b85a016c175623E: %self"} 
      134: !14 = distinct !{!14, !"_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$9rposition17h05b85a016c175623E"} 
      135: !15 = distinct !{!15, !14, !"_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$9rposition17h05b85a016c175623E: argument 1"} 
      136: !16 = !{!17} 
      137: !17 = distinct !{!17, !18, !"_ZN34issue_45964_bounds_check_slice_pos5rtest28_$u7b$$u7b$closure$u7d$$u7d$17h0964717175f51e31E: argument 1"} 
      138: !18 = distinct !{!18, !"_ZN34issue_45964_bounds_check_slice_pos5rtest28_$u7b$$u7b$closure$u7d$$u7d$17h0964717175f51e31E"} 
      139: !19 = !{!20, !13, !15} 
      140: !20 = distinct !{!20, !18, !"_ZN34issue_45964_bounds_check_slice_pos5rtest28_$u7b$$u7b$closure$u7d$$u7d$17h0964717175f51e31E: %_1"} 
------------------------------------------



