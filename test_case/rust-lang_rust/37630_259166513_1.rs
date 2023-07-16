
$ clang -emit-llvm -S --target=armv7-unknown-linux-gnueabihf foo.c -c -mfloat-abi=hard -O && cat foo.ll
; ModuleID = 'foo.c'
target datalayout = "e-m:e-p:32:32-i64:64-v128:64:128-a:0:32-n32-S64"
target triple = "armv7-unknown-linux-gnueabihf"

; Function Attrs: norecurse nounwind readnone
define double @a(double %b, i32 %c) #0 {
  %1 = tail call double @llvm.powi.f64(double %b, i32 %c)
  ret double %1
}

; Function Attrs: nounwind readnone
declare double @llvm.powi.f64(double, i32) #1

attributes #0 = { norecurse nounwind readnone "disable-tail-calls"="false" "less-precise-fpmad"="false" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-nans-fp-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="cortex-a8" "target-features"="+dsp,+neon,+vfp3" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #1 = { nounwind readnone }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 1, !"min_enum_size", i32 4}
!2 = !{!"clang version 3.8.0-2ubuntu4 (tags/RELEASE_380/final)"}
