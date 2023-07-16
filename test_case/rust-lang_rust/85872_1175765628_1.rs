llvm
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"unwind::libunwind::_Unwind_Exception" = type { i64, void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [6 x i64] }
%"unwind::libunwind::_Unwind_Context" = type { [0 x i8] }

define i16 @_ZN7example14u16_be_to_arch17hed7e5b3bb5283cd2E(i16 %0) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
  %data = alloca i16, align 2
  %1 = getelementptr inbounds i16, i16* %data, i64 1
  %2 = bitcast i16* %1 to i8*
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2) #4
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5) #4
  %_34.i.i = bitcast i16* %data to i8*
  %_39.i.i = getelementptr inbounds i8, i8* %2, i64 -1
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7) #4
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10) #4
  tail call void @llvm.experimental.noalias.scope.decl(metadata !12) #4
  tail call void @llvm.experimental.noalias.scope.decl(metadata !15) #4
  %3 = trunc i16 %0 to i8
  %4 = lshr i16 %0, 8
  %5 = trunc i16 %4 to i8
  store i8 %5, i8* %_34.i.i, align 2, !alias.scope !17, !noalias !20
  store i8 %3, i8* %_39.i.i, align 1, !alias.scope !21, !noalias !22
  %.sroa.0.0.copyload = load i16, i16* %data, align 2
  ret i16 %.sroa.0.0.copyload
}

define i32 @_ZN7example14u32_be_to_arch17h34475b177da247b9E(i32 %0) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
  %data.sroa.0.0.insert.insert = call i32 @llvm.bswap.i32(i32 %0)
  ret i32 %data.sroa.0.0.insert.insert
}

declare noundef i32 @rust_eh_personality(i32, i32 noundef, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*) unnamed_addr #1

declare void @llvm.experimental.noalias.scope.decl(metadata) #2

declare i32 @llvm.bswap.i32(i32) #3

attributes #0 = { nofree nosync nounwind nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #1 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #2 = { inaccessiblememonly nofree nosync nounwind willreturn }
attributes #3 = { nofree nosync nounwind readnone speculatable willreturn }
attributes #4 = { nounwind }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
!2 = !{!3}
!3 = distinct !{!3, !4, !"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$7reverse7revswap17heb06b5d2673f9130E: %a.0"}
!4 = distinct !{!4, !"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$7reverse7revswap17heb06b5d2673f9130E"}
!5 = !{!6}
!6 = distinct !{!6, !4, !"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$7reverse7revswap17heb06b5d2673f9130E: %b.0"}
!7 = !{!8}
!8 = distinct !{!8, !9, !"_ZN4core3mem4swap17h6e6b79a4a47ccb97E: %x"}
!9 = distinct !{!9, !"_ZN4core3mem4swap17h6e6b79a4a47ccb97E"}
!10 = !{!11}
!11 = distinct !{!11, !9, !"_ZN4core3mem4swap17h6e6b79a4a47ccb97E: %y"}
!12 = !{!13}
!13 = distinct !{!13, !14, !"_ZN4core3mem11swap_simple17h4fde363dcdbcde8aE: %x"}
!14 = distinct !{!14, !"_ZN4core3mem11swap_simple17h4fde363dcdbcde8aE"}
!15 = !{!16}
!16 = distinct !{!16, !14, !"_ZN4core3mem11swap_simple17h4fde363dcdbcde8aE: %y"}
!17 = !{!13, !8, !3, !18}
!18 = distinct !{!18, !19, !"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$7reverse17he43b2164665b0fd7E: %self.0"}
!19 = distinct !{!19, !"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$7reverse17he43b2164665b0fd7E"}
!20 = !{!16, !11, !6}
!21 = !{!16, !11, !6, !18}
!22 = !{!13, !8, !3}
