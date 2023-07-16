
; ModuleID = 'foo.3vww23kh-cgu.0'
source_filename = "foo.3vww23kh-cgu.0"
target datalayout = "e-m:e-p:32:32-i64:64-v128:64:128-a:0:32-n32-S64"
target triple = "armv7-none-linux-android"

%Header = type { [0 x i32], i8*, [0 x i32], i32, [0 x i32], i8*, [0 x i32], i32, [0 x i32] }
%HeaderIndices = type { [0 x i32], i32, [0 x i32], i32, [0 x i32], i32, [0 x i32], i32, [0 x i32] }
%"unwind::libunwind::_Unwind_Exception" = type { [0 x i64], i64, [0 x i32], void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [0 x i32], [20 x i32], [1 x i32] }
%"unwind::libunwind::_Unwind_Context" = type { [0 x i8] }

; foo::record_header_indices
; Function Attrs: nounwind nonlazybind uwtable
define void @_ZN3foo21record_header_indices17h1de73b969897f0d0E(i32 %bytes_ptr, %Header* nocapture readonly %headers, %HeaderIndices* nocapture %indices, i32 %len) unnamed_addr #0 personality i32 (i32, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %0 = icmp sgt i32 %len, 0
  br i1 %0, label %bb6.preheader, label %bb4

bb6.preheader:                                    ; preds = %start
  %min.iters.check = icmp ult i32 %len, 4
  br i1 %min.iters.check, label %bb6.preheader21, label %vector.memcheck

bb6.preheader21:                                  ; preds = %middle.block, %vector.memcheck, %bb6.preheader
  %iter.sroa.0.010.ph = phi i32 [ 0, %vector.memcheck ], [ 0, %bb6.preheader ], [ %n.vec, %middle.block ]
  br label %bb6

vector.memcheck:                                  ; preds = %bb6.preheader
  %scevgep = getelementptr %HeaderIndices, %HeaderIndices* %indices, i32 %len
  %scevgep14 = getelementptr %Header, %Header* %headers, i32 %len
  %1 = bitcast %Header* %scevgep14 to %HeaderIndices*
  %bound0 = icmp ugt %HeaderIndices* %1, %indices
  %2 = bitcast %HeaderIndices* %scevgep to %Header*
  %bound1 = icmp ugt %Header* %2, %headers
  %found.conflict = and i1 %bound0, %bound1
  br i1 %found.conflict, label %bb6.preheader21, label %vector.ph

vector.ph:                                        ; preds = %vector.memcheck
  %n.vec = and i32 %len, -4
  %broadcast.splatinsert19 = insertelement <4 x i32> undef, i32 %bytes_ptr, i32 0
  %broadcast.splat20 = shufflevector <4 x i32> %broadcast.splatinsert19, <4 x i32> undef, <4 x i32> zeroinitializer
  br label %vector.body

vector.body:                                      ; preds = %vector.body, %vector.ph
  %index = phi i32 [ 0, %vector.ph ], [ %index.next, %vector.body ]
  %3 = getelementptr inbounds %Header, %Header* %headers, i32 %index, i32 0, i32 0
  %4 = bitcast i32* %3 to <16 x i32>*
  %wide.vec = load <16 x i32>, <16 x i32>* %4, align 4
  %strided.vec = shufflevector <16 x i32> %wide.vec, <16 x i32> undef, <4 x i32> <i32 0, i32 4, i32 8, i32 12>
  %strided.vec16 = shufflevector <16 x i32> %wide.vec, <16 x i32> undef, <4 x i32> <i32 1, i32 5, i32 9, i32 13>
  %strided.vec17 = shufflevector <16 x i32> %wide.vec, <16 x i32> undef, <4 x i32> <i32 2, i32 6, i32 10, i32 14>
  %strided.vec18 = shufflevector <16 x i32> %wide.vec, <16 x i32> undef, <4 x i32> <i32 3, i32 7, i32 11, i32 15>
  %5 = sub <4 x i32> %strided.vec, %broadcast.splat20
  %6 = add <4 x i32> %5, %strided.vec16
  %7 = sub <4 x i32> %strided.vec17, %broadcast.splat20
  %8 = add <4 x i32> %7, %strided.vec18
  %9 = getelementptr inbounds %HeaderIndices, %HeaderIndices* %indices, i32 %index, i32 7
  %10 = getelementptr inbounds i32, i32* %9, i32 -3
  %11 = bitcast i32* %10 to <16 x i32>*
  %12 = shufflevector <4 x i32> %5, <4 x i32> %6, <8 x i32> <i32 0, i32 1, i32 2, i32 3, i32 4, i32 5, i32 6, i32 7>
  %13 = shufflevector <4 x i32> %7, <4 x i32> %8, <8 x i32> <i32 0, i32 1, i32 2, i32 3, i32 4, i32 5, i32 6, i32 7>
  %interleaved.vec = shufflevector <8 x i32> %12, <8 x i32> %13, <16 x i32> <i32 0, i32 4, i32 8, i32 12, i32 1, i32 5, i32 9, i32 13, i32 2, i32 6, i32 10, i32 14, i32 3, i32 7, i32 11, i32 15>
  store <16 x i32> %interleaved.vec, <16 x i32>* %11, align 4
  %index.next = add i32 %index, 4
  %14 = icmp eq i32 %index.next, %n.vec
  br i1 %14, label %middle.block, label %vector.body, !llvm.loop !1

middle.block:                                     ; preds = %vector.body
  %cmp.n = icmp eq i32 %n.vec, %len
  br i1 %cmp.n, label %bb4, label %bb6.preheader21

bb4:                                              ; preds = %bb6, %middle.block, %start
  ret void

bb6:                                              ; preds = %bb6.preheader21, %bb6
  %iter.sroa.0.010 = phi i32 [ %15, %bb6 ], [ %iter.sroa.0.010.ph, %bb6.preheader21 ]
  %15 = add nuw nsw i32 %iter.sroa.0.010, 1
  %16 = getelementptr inbounds %Header, %Header* %headers, i32 %iter.sroa.0.010, i32 0, i32 0
  %17 = load i32, i32* %16, align 4
  %18 = sub i32 %17, %bytes_ptr
  %19 = getelementptr inbounds %Header, %Header* %headers, i32 %iter.sroa.0.010, i32 3
  %20 = load i32, i32* %19, align 4
  %21 = add i32 %18, %20
  %22 = getelementptr inbounds %HeaderIndices, %HeaderIndices* %indices, i32 %iter.sroa.0.010, i32 0, i32 0
  store i32 %18, i32* %22, align 4
  %23 = getelementptr inbounds %HeaderIndices, %HeaderIndices* %indices, i32 %iter.sroa.0.010, i32 3
  store i32 %21, i32* %23, align 4
  %24 = getelementptr inbounds %Header, %Header* %headers, i32 %iter.sroa.0.010, i32 5
  %25 = bitcast i8** %24 to i32*
  %26 = load i32, i32* %25, align 4
  %27 = sub i32 %26, %bytes_ptr
  %28 = getelementptr inbounds %Header, %Header* %headers, i32 %iter.sroa.0.010, i32 7
  %29 = load i32, i32* %28, align 4
  %30 = add i32 %27, %29
  %31 = getelementptr inbounds %HeaderIndices, %HeaderIndices* %indices, i32 %iter.sroa.0.010, i32 5
  store i32 %27, i32* %31, align 4
  %32 = getelementptr inbounds %HeaderIndices, %HeaderIndices* %indices, i32 %iter.sroa.0.010, i32 7
  store i32 %30, i32* %32, align 4
  %exitcond = icmp eq i32 %15, %len
  br i1 %exitcond, label %bb4, label %bb6, !llvm.loop !3
}

; Function Attrs: nonlazybind uwtable
declare i32 @rust_eh_personality(i32, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*) unnamed_addr #1

attributes #0 = { nounwind nonlazybind uwtable "target-features"="+v7,+thumb-mode,+thumb2,+vfp3,+d16,-neon,+neon" }
attributes #1 = { nonlazybind uwtable "target-cpu"="generic" "target-features"="+v7,+thumb-mode,+thumb2,+vfp3,+d16,-neon,+neon" }

!llvm.module.flags = !{!0}

!0 = !{i32 2, !"RtLibUseGOT", i32 1}
!1 = distinct !{!1, !2}
!2 = !{!"llvm.loop.isvectorized", i32 1}
!3 = distinct !{!3, !2}`
