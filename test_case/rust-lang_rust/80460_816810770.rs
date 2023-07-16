ll
; ModuleID = 'playground.5mgwo5pt-cgu.0'
source_filename = "playground.5mgwo5pt-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"std::vec::Vec<i32>" = type { [0 x i64], { i32*, i64 }, [0 x i64], i64, [0 x i64] }
%"unwind::libunwind::_Unwind_Exception" = type { [0 x i64], i64, [0 x i64], void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [0 x i64], [6 x i64], [0 x i64] }
%"unwind::libunwind::_Unwind_Context" = type { [0 x i8] }

; playground::copy_slice_old
; Function Attrs: nonlazybind uwtable
define void @_ZN10playground14copy_slice_old17h6d0009fe79b83180E(%"std::vec::Vec<i32>"* noalias nocapture sret(%"std::vec::Vec<i32>") dereferenceable(24) %0, [0 x i32]* noalias nocapture nonnull readonly align 4 %slice.0, i64 %slice.1) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5), !noalias !8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10), !noalias !13
  tail call void @llvm.experimental.noalias.scope.decl(metadata !15), !noalias !18
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20), !noalias !23
  %1 = tail call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %slice.1, i64 4) #7
  %2 = extractvalue { i64, i1 } %1, 1
  %3 = extractvalue { i64, i1 } %1, 0
  %.sroa.3.0.i.i.i.i.i.i.i.i = select i1 %2, i64 0, i64 4
  %.sroa.0.0.i.i.i.i.i.i.i.i = select i1 %2, i64 undef, i64 %3
  br i1 %2, label %bb6.i.i.i.i.i.i.i, label %bb13.i.i.i.i.i.i.i

bb6.i.i.i.i.i.i.i:                                ; preds = %start
; call alloc::raw_vec::capacity_overflow
  tail call void @_ZN5alloc7raw_vec17capacity_overflow17h97153d40f6e2cd3bE(), !noalias !25
  unreachable

bb13.i.i.i.i.i.i.i:                               ; preds = %start
  %4 = icmp eq i64 %.sroa.0.0.i.i.i.i.i.i.i.i, 0
  br i1 %4, label %bb3.i.i.i.i.i.i.i.i.i, label %bb7.i.i.i.i.i.i.i.i.i

bb3.i.i.i.i.i.i.i.i.i:                            ; preds = %bb13.i.i.i.i.i.i.i
  %_2.i.i.i.i.i.i.i.i.i.i = inttoptr i64 %.sroa.3.0.i.i.i.i.i.i.i.i to i8*
  br label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17hdb6fd4f64ab5e4e5E.exit.i.i.i.i.i.i.i"

bb7.i.i.i.i.i.i.i.i.i:                            ; preds = %bb13.i.i.i.i.i.i.i
  %5 = tail call i8* @__rust_alloc(i64 %.sroa.0.0.i.i.i.i.i.i.i.i, i64 %.sroa.3.0.i.i.i.i.i.i.i.i) #7, !noalias !25
  %6 = icmp eq i8* %5, null
  %.sroa.0.0.i.op.i.i.i.i.i.i.i = lshr i64 %.sroa.0.0.i.i.i.i.i.i.i.i, 2
  %phi.bo.i.i.i.i.i.i.i = select i1 %6, i64 0, i64 %.sroa.0.0.i.op.i.i.i.i.i.i.i
  br label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17hdb6fd4f64ab5e4e5E.exit.i.i.i.i.i.i.i"

"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17hdb6fd4f64ab5e4e5E.exit.i.i.i.i.i.i.i": ; preds = %bb7.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i.i
  %.sroa.4.0.i.i.i.i.i.i.i.i.i = phi i64 [ 0, %bb3.i.i.i.i.i.i.i.i.i ], [ %phi.bo.i.i.i.i.i.i.i, %bb7.i.i.i.i.i.i.i.i.i ]
  %.sroa.0.0.i.i.i.i.i.i.i.i.i = phi i8* [ %_2.i.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i.i ], [ %5, %bb7.i.i.i.i.i.i.i.i.i ]
  %7 = icmp eq i8* %.sroa.0.0.i.i.i.i.i.i.i.i.i, null
  br i1 %7, label %bb20.i.i.i.i.i.i.i, label %"_ZN5alloc5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$6to_vec17h82a8a1f74f9ea9ddE.exit"

bb20.i.i.i.i.i.i.i:                               ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17hdb6fd4f64ab5e4e5E.exit.i.i.i.i.i.i.i"
; call alloc::alloc::handle_alloc_error
  tail call void @_ZN5alloc5alloc18handle_alloc_error17hbbcab09c85c3ddd4E(i64 %.sroa.0.0.i.i.i.i.i.i.i.i, i64 %.sroa.3.0.i.i.i.i.i.i.i.i), !noalias !25
  unreachable

"_ZN5alloc5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$6to_vec17h82a8a1f74f9ea9ddE.exit": ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17hdb6fd4f64ab5e4e5E.exit.i.i.i.i.i.i.i"
  %8 = bitcast %"std::vec::Vec<i32>"* %0 to i8**
  store i8* %.sroa.0.0.i.i.i.i.i.i.i.i.i, i8** %8, align 8, !alias.scope !26, !noalias !27
  %9 = getelementptr inbounds %"std::vec::Vec<i32>", %"std::vec::Vec<i32>"* %0, i64 0, i32 1, i32 1
  store i64 %.sroa.4.0.i.i.i.i.i.i.i.i.i, i64* %9, align 8, !alias.scope !26, !noalias !27
  %10 = getelementptr inbounds %"std::vec::Vec<i32>", %"std::vec::Vec<i32>"* %0, i64 0, i32 3
  %11 = shl nuw i64 %slice.1, 2
  %12 = bitcast [0 x i32]* %slice.0 to i8*
  tail call void @llvm.memcpy.p0i8.p0i8.i64(i8* nonnull align 4 %.sroa.0.0.i.i.i.i.i.i.i.i.i, i8* nonnull align 4 %12, i64 %11, i1 false) #7, !noalias !28
  store i64 %slice.1, i64* %10, align 8, !alias.scope !29, !noalias !27
  ret void
}

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare { i64, i1 } @llvm.umul.with.overflow.i64(i64, i64) #1

; Function Attrs: nounwind nonlazybind uwtable
declare i32 @rust_eh_personality(i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*) unnamed_addr #2

; Function Attrs: argmemonly nofree nosync nounwind willreturn
declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #3

; Function Attrs: nounwind nonlazybind uwtable
declare noalias i8* @__rust_alloc(i64, i64) unnamed_addr #2

; alloc::raw_vec::capacity_overflow
; Function Attrs: noreturn nonlazybind uwtable
declare void @_ZN5alloc7raw_vec17capacity_overflow17h97153d40f6e2cd3bE() unnamed_addr #4

; alloc::alloc::handle_alloc_error
; Function Attrs: cold noreturn nounwind nonlazybind uwtable
declare void @_ZN5alloc5alloc18handle_alloc_error17hbbcab09c85c3ddd4E(i64, i64) unnamed_addr #5

; Function Attrs: inaccessiblememonly nofree nosync nounwind willreturn
declare void @llvm.experimental.noalias.scope.decl(metadata) #6

attributes #0 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #1 = { nofree nosync nounwind readnone speculatable willreturn }
attributes #2 = { nounwind nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #3 = { argmemonly nofree nosync nounwind willreturn }
attributes #4 = { noreturn nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #5 = { cold noreturn nounwind nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #6 = { inaccessiblememonly nofree nosync nounwind willreturn }
attributes #7 = { nounwind }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
!2 = !{!3}
!3 = distinct !{!3, !4, !"_ZN5alloc5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$6to_vec17h82a8a1f74f9ea9ddE: argument 0"}
!4 = distinct !{!4, !"_ZN5alloc5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$6to_vec17h82a8a1f74f9ea9ddE"}
!5 = !{!6}
!6 = distinct !{!6, !7, !"_ZN5alloc5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$9to_vec_in17hf0efe825ef187949E: argument 0"}
!7 = distinct !{!7, !"_ZN5alloc5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$9to_vec_in17hf0efe825ef187949E"}
!8 = !{!3, !9}
!9 = distinct !{!9, !4, !"_ZN5alloc5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$6to_vec17h82a8a1f74f9ea9ddE: %self.0"}
!10 = !{!11}
!11 = distinct !{!11, !12, !"_ZN5alloc5slice4hack6to_vec17h4e5b681653d8de3eE: argument 0"}
!12 = distinct !{!12, !"_ZN5alloc5slice4hack6to_vec17h4e5b681653d8de3eE"}
!13 = !{!6, !14, !3, !9}
!14 = distinct !{!14, !7, !"_ZN5alloc5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$9to_vec_in17hf0efe825ef187949E: %self.0"}
!15 = !{!16}
!16 = distinct !{!16, !17, !"_ZN52_$LT$T$u20$as$u20$alloc..slice..hack..ConvertVec$GT$6to_vec17h153ad63a55993150E: %v"}
!17 = distinct !{!17, !"_ZN52_$LT$T$u20$as$u20$alloc..slice..hack..ConvertVec$GT$6to_vec17h153ad63a55993150E"}
!18 = !{!11, !19, !6, !14, !3, !9}
!19 = distinct !{!19, !12, !"_ZN5alloc5slice4hack6to_vec17h4e5b681653d8de3eE: %s.0"}
!20 = !{!21}
!21 = distinct !{!21, !22, !"_ZN5alloc3vec16Vec$LT$T$C$A$GT$16with_capacity_in17hce1a244487fd289dE: argument 0"}
!22 = distinct !{!22, !"_ZN5alloc3vec16Vec$LT$T$C$A$GT$16with_capacity_in17hce1a244487fd289dE"}
!23 = !{!16, !24, !11, !19, !6, !14, !3, !9}
!24 = distinct !{!24, !17, !"_ZN52_$LT$T$u20$as$u20$alloc..slice..hack..ConvertVec$GT$6to_vec17h153ad63a55993150E: %s.0"}
!25 = !{!21, !16, !24, !11, !19, !6, !14, !3, !9}
!26 = !{!21, !16, !11, !6, !3}
!27 = !{!24, !19, !14, !9}
!28 = !{!16, !11, !6, !3}
!29 = !{!30, !16, !11, !6, !3}
!30 = distinct !{!30, !31, !"_ZN5alloc3vec16Vec$LT$T$C$A$GT$7set_len17hcd51c7b209261a71E: %self"}
!31 = distinct !{!31, !"_ZN5alloc3vec16Vec$LT$T$C$A$GT$7set_len17hcd51c7b209261a71E"}
