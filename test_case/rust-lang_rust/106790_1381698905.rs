plain

running 399 tests
i......i.............i....i..ii.................iii........iii.i........i............... 88/399
....ii.................i............i..i................i...i....iii.......i..i.....i.ii 176/399
ii..iii.F......i.iii....i...i.....................i....ii..i.....ii..i.ii....i.......... 264/399
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
...................iiiiiiiii...................
failures:


---- [codegen] checkout/tests/codegen/issue-103840.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-103840/issue-103840.ll" "/checkout/tests/codegen/issue-103840.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/tests/codegen/issue-103840.rs:5:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: __rust_dealloc
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-103840/issue-103840.ll:73:18: note: found here
 tail call void @__rust_dealloc(i8* nonnull %1, i64 %_16.0.i.i.i.i.i, i64 8) #9, !noalias !17

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-103840/issue-103840.ll
Check file: /checkout/tests/codegen/issue-103840.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
       1: ; ModuleID = 'issue_103840.f9de1d84-cgu.0' 
       2: source_filename = "issue_103840.f9de1d84-cgu.0" 
       3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
       4: target triple = "x86_64-unknown-linux-gnu" 
       5:  
       6: %"alloc::vec::Vec<usize>" = type { { i64*, i64 }, i64 } 
       7: %"core::panic::location::Location<'_>" = type { { [0 x i8]*, i64 }, i32, i32 } 
       8: %"unwind::libunwind::_Unwind_Exception" = type { i64, void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [2 x i64] } 
       9: %"unwind::libunwind::_Unwind_Context" = type { [0 x i8] } 
      10:  
      11: @alloc116 = private unnamed_addr constant <{ [101 x i8] }> <{ [101 x i8] c"unsafe precondition(s) violated: ptr::read requires that the pointer argument is aligned and non-null" }>, align 1 
      12: @alloc125 = private unnamed_addr constant <{ [30 x i8] }> <{ [30 x i8] c"assertion failed: self.is_ok()" }>, align 1 
      13: @alloc126 = private unnamed_addr constant <{ [38 x i8] }> <{ [38 x i8] c"/checkout/library/alloc/src/raw_vec.rs" }>, align 1 
      14: @alloc127 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [38 x i8] }>, <{ [38 x i8] }>* @alloc126, i32 0, i32 0, i32 0), [16 x i8] c"&\00\00\00\00\00\00\00\08\01\00\00=\00\00\00" }>, align 8 
      15: @0 = private unnamed_addr constant <{ [16 x i8] }> <{ [16 x i8] c"\08\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00" }>, align 8 
      16:  
      17: ; issue_103840::foo 
      18: ; Function Attrs: nonlazybind uwtable 
      19: define void @_ZN12issue_1038403foo17h98bc324ac70a64fbE(%"alloc::vec::Vec<usize>"* noalias nocapture noundef align 8 dereferenceable(24) %t) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality { 
      20: start: 
      21:  %_1.0.i = load i64*, i64** bitcast (<{ [16 x i8] }>* @0 to i64**), align 8, !noalias !2, !nonnull !5, !noundef !5 
      22:  tail call void @llvm.experimental.noalias.scope.decl(metadata !6) 
      23:  tail call void @llvm.experimental.noalias.scope.decl(metadata !9) 
      24:  %taken.sroa.0.0.tmp.sroa.0.0..sroa_cast.i.i.sroa_idx = getelementptr inbounds %"alloc::vec::Vec<usize>", %"alloc::vec::Vec<usize>"* %t, i64 0, i32 0, i32 0 
      25:  %taken.sroa.0.0.copyload23 = load i64*, i64** %taken.sroa.0.0.tmp.sroa.0.0..sroa_cast.i.i.sroa_idx, align 8, !alias.scope !11, !noalias !9 
      26:  %taken.sroa.5.0.tmp.sroa.0.0..sroa_cast.i.i.sroa_idx26 = getelementptr inbounds %"alloc::vec::Vec<usize>", %"alloc::vec::Vec<usize>"* %t, i64 0, i32 0, i32 1 
      27:  %taken.sroa.5.0.copyload27 = load i64, i64* %taken.sroa.5.0.tmp.sroa.0.0..sroa_cast.i.i.sroa_idx26, align 8, !alias.scope !11, !noalias !9 
      28:  %taken.sroa.528.0.tmp.sroa.0.0..sroa_cast.i.i.sroa_idx31 = getelementptr inbounds %"alloc::vec::Vec<usize>", %"alloc::vec::Vec<usize>"* %t, i64 0, i32 1 
      29:  %taken.sroa.528.0.copyload32 = load i64, i64* %taken.sroa.528.0.tmp.sroa.0.0..sroa_cast.i.i.sroa_idx31, align 8, !alias.scope !11, !noalias !9 
      30:  store i64* %_1.0.i, i64** %taken.sroa.0.0.tmp.sroa.0.0..sroa_cast.i.i.sroa_idx, align 8, !alias.scope !13, !noalias !6 
      31:  %_9.sroa.4.0.dest13.i.sroa_cast = bitcast i64* %taken.sroa.5.0.tmp.sroa.0.0..sroa_cast.i.i.sroa_idx26 to i8* 
      32:  call void @llvm.memset.p0i8.i64(i8* noundef nonnull align 8 dereferenceable(16) %_9.sroa.4.0.dest13.i.sroa_cast, i8 0, i64 16, i1 false) 
      33:  %0 = icmp eq i64 %taken.sroa.528.0.copyload32, 0 
      34:  %1 = bitcast i64* %_1.0.i to i8* 
      35:  br i1 %0, label %bb1, label %bb2.i 
      36:  
      37: bb2.i: ; preds = %start 
      38:  %_3.0.i = add i64 %taken.sroa.528.0.copyload32, -1 
      39:  %2 = icmp ne i64* %taken.sroa.0.0.copyload23, null 
      40:  call void @llvm.assume(i1 %2) 
      41:  %3 = getelementptr inbounds i64, i64* %taken.sroa.0.0.copyload23, i64 %_3.0.i 
      42:  %4 = ptrtoint i64* %3 to i64 
      43:  %_28.i.i.i.i = and i64 %4, 7 
      44:  %5 = icmp eq i64 %_28.i.i.i.i, 0 
      45:  br i1 %5, label %bb1, label %bb2.i.i 
      46:  
      47: bb2.i.i: ; preds = %bb2.i 
      48: ; call core::panicking::panic_nounwind 
      49:  tail call void @_ZN4core9panicking14panic_nounwind17h71ff1a09dbd48e12E([0 x i8]* noalias noundef nonnull readonly align 1 bitcast (<{ [101 x i8] }>* @alloc116 to [0 x i8]*), i64 101) #7, !noalias !14 
      50:  unreachable 
      51:  
      52: bb1: ; preds = %bb2.i, %start 
      53:  %taken.sroa.528.0 = phi i64 [ 0, %start ], [ %_3.0.i, %bb2.i ] 
      54:  tail call void @llvm.experimental.noalias.scope.decl(metadata !17) 
      55:  %_15.i.i.i.i = load i64, i64* %taken.sroa.5.0.tmp.sroa.0.0..sroa_cast.i.i.sroa_idx26, align 8, !alias.scope !20, !noalias !23 
      56:  %_3.i.i.i.i = icmp eq i64 %_15.i.i.i.i, 0 
      57:  br i1 %_3.i.i.i.i, label %bb4, label %bb5.i.i.i.i 
      58:  
      59: bb5.i.i.i.i: ; preds = %bb1 
      60:  %_6.i.i.i.i.i = icmp ugt i64 %_15.i.i.i.i, 1152921504606846975 
      61:  br i1 %_6.i.i.i.i.i, label %bb2.i.i.i.i.i, label %bb2.i.i.i 
      62:  
      63: bb2.i.i.i.i.i: ; preds = %bb5.i.i.i.i 
      64: ; invoke core::panicking::panic 
      65:  invoke void @_ZN4core9panicking5panic17h9d8d4750127fbc52E([0 x i8]* noalias noundef nonnull readonly align 1 bitcast (<{ [30 x i8] }>* @alloc125 to [0 x i8]*), i64 30, %"core::panic::location::Location<'_>"* noalias noundef readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc127 to %"core::panic::location::Location<'_>"*)) #8 
      66:  to label %.noexc unwind label %bb2 
      67:  
      68: .noexc: ; preds = %bb2.i.i.i.i.i 
      69:  unreachable 
      70:  
      71: bb2.i.i.i: ; preds = %bb5.i.i.i.i 
      72:  %_16.0.i.i.i.i.i = shl nuw nsw i64 %_15.i.i.i.i, 3 
      73:  tail call void @__rust_dealloc(i8* nonnull %1, i64 %_16.0.i.i.i.i.i, i64 8) #9, !noalias !17 
not:5                      !~~~~~~~~~~~~~                                                                error: no match expected
      74:  br label %bb4 
      75:  
      76: bb4: ; preds = %bb2.i.i.i, %bb1 
      77:  store i64* %taken.sroa.0.0.copyload23, i64** %taken.sroa.0.0.tmp.sroa.0.0..sroa_cast.i.i.sroa_idx, align 8 
      78:  store i64 %taken.sroa.5.0.copyload27, i64* %taken.sroa.5.0.tmp.sroa.0.0..sroa_cast.i.i.sroa_idx26, align 8 
      79:  store i64 %taken.sroa.528.0, i64* %taken.sroa.528.0.tmp.sroa.0.0..sroa_cast.i.i.sroa_idx31, align 8 
      80:  ret void 
      81:  
      82: bb2: ; preds = %bb2.i.i.i.i.i 
      83:  %6 = landingpad { i8*, i32 } 
      84:  cleanup 
      85:  store i64* %taken.sroa.0.0.copyload23, i64** %taken.sroa.0.0.tmp.sroa.0.0..sroa_cast.i.i.sroa_idx, align 8 
      86:  store i64 %taken.sroa.5.0.copyload27, i64* %taken.sroa.5.0.tmp.sroa.0.0..sroa_cast.i.i.sroa_idx26, align 8 
      87:  store i64 %taken.sroa.528.0, i64* %taken.sroa.528.0.tmp.sroa.0.0..sroa_cast.i.i.sroa_idx31, align 8 
      88:  resume { i8*, i32 } %6 
      89: } 
      90:  
      91: ; Function Attrs: nonlazybind uwtable 
      92: declare noundef i32 @rust_eh_personality(i32, i32 noundef, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*) unnamed_addr #0 
      94: ; core::panicking::panic_nounwind 
      94: ; core::panicking::panic_nounwind 
      95: ; Function Attrs: cold noinline noreturn nounwind nonlazybind uwtable 
      96: declare void @_ZN4core9panicking14panic_nounwind17h71ff1a09dbd48e12E([0 x i8]* noalias noundef nonnull readonly align 1, i64) unnamed_addr #1 
      97:  
      98: ; Function Attrs: inaccessiblememonly mustprogress nofree nosync nounwind willreturn 
      99: declare void @llvm.assume(i1 noundef) #2 
     101: ; core::panicking::panic 
     101: ; core::panicking::panic 
     102: ; Function Attrs: cold noinline noreturn nonlazybind uwtable 
     103: declare void @_ZN4core9panicking5panic17h9d8d4750127fbc52E([0 x i8]* noalias noundef nonnull readonly align 1, i64, %"core::panic::location::Location<'_>"* noalias noundef readonly align 8 dereferenceable(24)) unnamed_addr #3 
     104:  
     105: ; Function Attrs: nounwind nonlazybind uwtable 
     106: declare void @__rust_dealloc(i8*, i64, i64) unnamed_addr #4 
     107:  
     108: ; Function Attrs: argmemonly nofree nounwind willreturn writeonly 
     109: declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #5 
     110:  
     111: ; Function Attrs: inaccessiblememonly nofree nosync nounwind willreturn 
     112: declare void @llvm.experimental.noalias.scope.decl(metadata) #6 
     113:  
     114: attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
     115: attributes #1 = { cold noinline noreturn nounwind nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
     116: attributes #2 = { inaccessiblememonly mustprogress nofree nosync nounwind willreturn } 
     117: attributes #3 = { cold noinline noreturn nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
     118: attributes #4 = { nounwind nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
     119: attributes #5 = { argmemonly nofree nounwind willreturn writeonly } 
     120: attributes #6 = { inaccessiblememonly nofree nosync nounwind willreturn } 
     121: attributes #7 = { noreturn nounwind } 
     122: attributes #8 = { noreturn } 
     123: attributes #9 = { nounwind } 
     124:  
     125: !llvm.module.flags = !{!0, !1} 
     126:  
     127: !0 = !{i32 7, !"PIC Level", i32 2} 
     128: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
     129: !2 = !{!3} 
     130: !3 = distinct !{!3, !4, !"_ZN67_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..default..Default$GT$7default17h11095d963a66aadbE: argument 0"} 
     131: !4 = distinct !{!4, !"_ZN67_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..default..Default$GT$7default17h11095d963a66aadbE"} 
     132: !5 = !{} 
     133: !6 = !{!7} 
     134: !7 = distinct !{!7, !8, !"_ZN4core3mem7replace17h4b03d26be75e73ffE: %result"} 
     135: !8 = distinct !{!8, !"_ZN4core3mem7replace17h4b03d26be75e73ffE"} 
     136: !9 = !{!10} 
     137: !10 = distinct !{!10, !8, !"_ZN4core3mem7replace17h4b03d26be75e73ffE: %src"} 
     138: !11 = !{!7, !12} 
     139: !12 = distinct !{!12, !8, !"_ZN4core3mem7replace17h4b03d26be75e73ffE: %dest"} 
     140: !13 = !{!12, !10} 
     141: !14 = !{!15} 
     142: !15 = distinct !{!15, !16, !"_ZN5alloc3vec16Vec$LT$T$C$A$GT$3pop17h19bfcf4edbd6e6daE: %self"} 
     143: !16 = distinct !{!16, !"_ZN5alloc3vec16Vec$LT$T$C$A$GT$3pop17h19bfcf4edbd6e6daE"} 
     144: !17 = !{!18} 
     145: !18 = distinct !{!18, !19, !"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17he7db0eef558b6356E: %self"} 
     146: !19 = distinct !{!19, !"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17he7db0eef558b6356E"} 
     147: !20 = !{!21, !18} 
     148: !21 = distinct !{!21, !22, !"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h727cb53aab3d6da7E: %self"} 
     149: !22 = distinct !{!22, !"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h727cb53aab3d6da7E"} 
     150: !23 = !{!24} 
     151: !24 = distinct !{!24, !22, !"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h727cb53aab3d6da7E: argument 0"} 
------------------------------------------



