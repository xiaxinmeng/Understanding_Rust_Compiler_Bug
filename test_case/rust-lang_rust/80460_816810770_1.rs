ll
; ModuleID = 'playground.5mgwo5pt-cgu.0'
source_filename = "playground.5mgwo5pt-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"std::vec::Vec<i32>" = type { [0 x i64], { i32*, i64 }, [0 x i64], i64, [0 x i64] }
%"unwind::libunwind::_Unwind_Exception" = type { [0 x i64], i64, [0 x i64], void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [0 x i64], [6 x i64], [0 x i64] }
%"unwind::libunwind::_Unwind_Context" = type { [0 x i8] }

; playground::copy_slice_new
; Function Attrs: nonlazybind uwtable
define void @_ZN10playground14copy_slice_new17hbe14d17b96488007E(%"std::vec::Vec<i32>"* noalias nocapture sret(%"std::vec::Vec<i32>") dereferenceable(24) %0, [0 x i32]* noalias nocapture nonnull readonly align 4 %slice.0, i64 %slice.1) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2)
  %1 = tail call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %slice.1, i64 4) #7
  %2 = extractvalue { i64, i1 } %1, 1
  %3 = extractvalue { i64, i1 } %1, 0
  %.sroa.3.0.i.i.i.i.i.i.i = select i1 %2, i64 0, i64 4
  %.sroa.0.0.i.i.i.i.i.i.i = select i1 %2, i64 undef, i64 %3
  br i1 %2, label %bb6.i.i.i.i.i.i, label %bb13.i.i.i.i.i.i

bb6.i.i.i.i.i.i:                                  ; preds = %start
; call alloc::raw_vec::capacity_overflow
  tail call void @_ZN5alloc7raw_vec17capacity_overflow17h97153d40f6e2cd3bE(), !noalias !5
  unreachable

bb13.i.i.i.i.i.i:                                 ; preds = %start
  %4 = icmp eq i64 %.sroa.0.0.i.i.i.i.i.i.i, 0
  br i1 %4, label %bb3.i.i.i.i.i.i.i.i, label %bb7.i.i.i.i.i.i.i.i

bb3.i.i.i.i.i.i.i.i:                              ; preds = %bb13.i.i.i.i.i.i
  %_2.i.i.i.i.i.i.i.i.i = inttoptr i64 %.sroa.3.0.i.i.i.i.i.i.i to i8*
  br label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17hdb6fd4f64ab5e4e5E.exit.i.i.i.i.i.i"

bb7.i.i.i.i.i.i.i.i:                              ; preds = %bb13.i.i.i.i.i.i
  %5 = tail call i8* @__rust_alloc(i64 %.sroa.0.0.i.i.i.i.i.i.i, i64 %.sroa.3.0.i.i.i.i.i.i.i) #7, !noalias !5
  br label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17hdb6fd4f64ab5e4e5E.exit.i.i.i.i.i.i"

"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17hdb6fd4f64ab5e4e5E.exit.i.i.i.i.i.i": ; preds = %bb7.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i
  %.sroa.0.0.i.i.i.i.i.i.i.i = phi i8* [ %_2.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i ], [ %5, %bb7.i.i.i.i.i.i.i.i ]
  %6 = icmp eq i8* %.sroa.0.0.i.i.i.i.i.i.i.i, null
  br i1 %6, label %bb20.i.i.i.i.i.i, label %_ZN10playground4hack6to_vec17h302c903ca456a4daE.exit

bb20.i.i.i.i.i.i:                                 ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17hdb6fd4f64ab5e4e5E.exit.i.i.i.i.i.i"
; call alloc::alloc::handle_alloc_error
  tail call void @_ZN5alloc5alloc18handle_alloc_error17hbbcab09c85c3ddd4E(i64 %.sroa.0.0.i.i.i.i.i.i.i, i64 %.sroa.3.0.i.i.i.i.i.i.i), !noalias !5
  unreachable

_ZN10playground4hack6to_vec17h302c903ca456a4daE.exit: ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17hdb6fd4f64ab5e4e5E.exit.i.i.i.i.i.i"
  %7 = shl nuw i64 %slice.1, 2
  %8 = bitcast [0 x i32]* %slice.0 to i8*
  tail call void @llvm.memcpy.p0i8.p0i8.i64(i8* nonnull align 4 %.sroa.0.0.i.i.i.i.i.i.i.i, i8* nonnull align 4 %8, i64 %7, i1 false) #7, !noalias !2
  %9 = bitcast %"std::vec::Vec<i32>"* %0 to i8**
  store i8* %.sroa.0.0.i.i.i.i.i.i.i.i, i8** %9, align 8, !alias.scope !11, !noalias !16
  %10 = getelementptr inbounds %"std::vec::Vec<i32>", %"std::vec::Vec<i32>"* %0, i64 0, i32 1, i32 1
  store i64 %slice.1, i64* %10, align 8, !alias.scope !11, !noalias !16
  %11 = getelementptr inbounds %"std::vec::Vec<i32>", %"std::vec::Vec<i32>"* %0, i64 0, i32 3
  store i64 %slice.1, i64* %11, align 8, !alias.scope !11, !noalias !16
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
!3 = distinct !{!3, !4, !"_ZN10playground4hack6to_vec17h302c903ca456a4daE: argument 0"}
!4 = distinct !{!4, !"_ZN10playground4hack6to_vec17h302c903ca456a4daE"}
!5 = !{!6, !8, !3, !10}
!6 = distinct !{!6, !7, !"_ZN52_$LT$T$u20$as$u20$playground..hack..ConvertBoxed$GT$14to_boxed_slice17hf040d5891f80ff7fE: %s.0"}
!7 = distinct !{!7, !"_ZN52_$LT$T$u20$as$u20$playground..hack..ConvertBoxed$GT$14to_boxed_slice17hf040d5891f80ff7fE"}
!8 = distinct !{!8, !9, !"_ZN10playground4hack14to_boxed_slice17hee7857f846790a13E: %s.0"}
!9 = distinct !{!9, !"_ZN10playground4hack14to_boxed_slice17hee7857f846790a13E"}
!10 = distinct !{!10, !4, !"_ZN10playground4hack6to_vec17h302c903ca456a4daE: %s.0"}
!11 = !{!12, !14, !3}
!12 = distinct !{!12, !13, !"_ZN5alloc3vec16Vec$LT$T$C$A$GT$17from_raw_parts_in17h851da6b596063aefE: argument 0"}
!13 = distinct !{!13, !"_ZN5alloc3vec16Vec$LT$T$C$A$GT$17from_raw_parts_in17h851da6b596063aefE"}
!14 = distinct !{!14, !15, !"_ZN10playground4hack8into_vec17hf50de4bbad45c759E: argument 0"}
!15 = distinct !{!15, !"_ZN10playground4hack8into_vec17hf50de4bbad45c759E"}
!16 = !{!17, !10}
!17 = distinct !{!17, !15, !"_ZN10playground4hack8into_vec17hf50de4bbad45c759E: argument 1"}
