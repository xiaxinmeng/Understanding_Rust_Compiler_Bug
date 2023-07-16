
; ModuleID = 'example.bxq6xejs-cgu.0'
source_filename = "example.bxq6xejs-cgu.0"
target datalayout = "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64"
target triple = "thumbv7em-none-unknown-eabi"

; Function Attrs: minsize noreturn nounwind optsize
define void @_start() unnamed_addr #0 {
start:
  %e.i = alloca {}, align 1
  %0 = bitcast {}* %e.i to i8*
  call void @llvm.lifetime.start.p0i8(i64 0, i8* nonnull %0)
; call core::result::unwrap_failed
  call fastcc void @_ZN4core6result13unwrap_failed17h28e5bee20479b83aE({}* nonnull align 1 %e.i) #4
  unreachable
}

; Function Attrs: argmemonly nounwind willreturn
declare void @llvm.lifetime.start.p0i8(i64 immarg, i8* nocapture) #1

; Function Attrs: inaccessiblememonly nounwind willreturn
declare void @llvm.sideeffect() #2

; core::result::unwrap_failed
; Function Attrs: cold noinline noreturn nounwind
define internal fastcc void @_ZN4core6result13unwrap_failed17h28e5bee20479b83aE({}* nocapture nonnull readnone align 1 %0) unnamed_addr #3 {
; call core::panicking::panic_fmt
  tail call fastcc void @_ZN4core9panicking9panic_fmt17hbd2067f8be3a4464E()
  unreachable
}

; core::panicking::panic_fmt
; Function Attrs: cold noinline noreturn nounwind
define internal fastcc void @_ZN4core9panicking9panic_fmt17hbd2067f8be3a4464E() unnamed_addr #3 {
  br label %bb1.i

bb1.i:                                            ; preds = %bb1.i, %0
  tail call void @llvm.sideeffect() #4
  br label %bb1.i
}

attributes #0 = { minsize noreturn nounwind optsize "frame-pointer"="all" "target-cpu"="generic" }
attributes #1 = { argmemonly nounwind willreturn }
attributes #2 = { inaccessiblememonly nounwind willreturn }
attributes #3 = { cold noinline noreturn nounwind "frame-pointer"="all" "target-cpu"="generic" }
attributes #4 = { nounwind }
