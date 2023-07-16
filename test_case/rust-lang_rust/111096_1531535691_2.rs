llvm-ir
; ModuleID = 'test_c.46e36185cb05fdac-cgu.0'
source_filename = "test_c.46e36185cb05fdac-cgu.0"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc"

%"core::num::error::TryFromIntError" = type { {} }

@alloc_00ae4b301f7fab8ac9617c03fcbd7274 = private unnamed_addr constant <{ [43 x i8] }> <{ [43 x i8] c"called `Result::unwrap()` on an `Err` value" }>, align 1
@vtable.0 = private unnamed_addr constant <{ ptr, [16 x i8], ptr }> <{ ptr @"_ZN4core3ptr54drop_in_place$LT$core..num..error..TryFromIntError$GT$17h7f451cb6d2235960E", [16 x i8] c"\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", ptr @"_ZN70_$LT$core..num..error..TryFromIntError$u20$as$u20$core..fmt..Debug$GT$3fmt17hc75af1d3d73262e8E" }>, align 8
@alloc_2bbc63c594421bee4d4201fca631d215 = private unnamed_addr constant <{ [9 x i8] }> <{ [9 x i8] c"test_c.rs" }>, align 1
@alloc_ea0d6a17a859d145cd5d93e8f6368c86 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_2bbc63c594421bee4d4201fca631d215, [16 x i8] c"\09\00\00\00\00\00\00\00\04\00\00\00\12\00\00\00" }>, align 8

; core::ptr::drop_in_place<core::num::error::TryFromIntError>
; Function Attrs: inlinehint mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable
define internal void @"_ZN4core3ptr54drop_in_place$LT$core..num..error..TryFromIntError$GT$17h7f451cb6d2235960E"(ptr nocapture readnone %_1) unnamed_addr #0 {
start:
  ret void
}

; Function Attrs: uwtable
define noundef i32 @cast(i64 noundef %v) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
start:
  %e.i = alloca %"core::num::error::TryFromIntError", align 1
  %_5.0 = icmp ugt i64 %v, 4294967295
  br i1 %_5.0, label %bb5.split, label %bb6.split

bb6.split:                                        ; preds = %start
  %_8 = trunc i64 %v to i32
  ret i32 %_8

bb5.split:                                        ; preds = %start
  call void @llvm.lifetime.start.p0(i64 0, ptr nonnull %e.i)
; call core::result::unwrap_failed
  call void @_ZN4core6result13unwrap_failed17h7fc0894136e1312bE(ptr noalias noundef nonnull readonly align 1 @alloc_00ae4b301f7fab8ac9617c03fcbd7274, i64 noundef 43, ptr noundef nonnull align 1 %e.i, ptr noalias noundef nonnull readonly align 8 dereferenceable(24) @vtable.0, ptr noalias noundef nonnull readonly align 8 dereferenceable(24) @alloc_ea0d6a17a859d145cd5d93e8f6368c86) #5
  unreachable
}

declare i32 @__CxxFrameHandler3(...) unnamed_addr #2

; <core::num::error::TryFromIntError as core::fmt::Debug>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @"_ZN70_$LT$core..num..error..TryFromIntError$u20$as$u20$core..fmt..Debug$GT$3fmt17hc75af1d3d73262e8E"(ptr noalias noundef nonnull readonly align 1, ptr noalias noundef align 8 dereferenceable(64)) unnamed_addr #1

; core::result::unwrap_failed
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core6result13unwrap_failed17h7fc0894136e1312bE(ptr noalias noundef nonnull readonly align 1, i64 noundef, ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 dereferenceable(24), ptr noalias noundef readonly align 8 dereferenceable(24)) unnamed_addr #3

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr nocapture) #4

attributes #0 = { inlinehint mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable "target-cpu"="x86-64" }
attributes #1 = { uwtable "target-cpu"="x86-64" }
attributes #2 = { "target-cpu"="x86-64" }
attributes #3 = { cold noinline noreturn uwtable "target-cpu"="x86-64" }
attributes #4 = { nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #5 = { noreturn }

!llvm.module.flags = !{!0}

!0 = !{i32 8, !"PIC Level", i32 2}
