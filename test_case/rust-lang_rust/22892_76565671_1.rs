 llvm
; ModuleID = 'mintest.c'
target datalayout = "e-m:e-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: nounwind uwtable
define i32 @main(i32 %argc, i8** %argv) #0 {
  %1 = alloca i32, align 4
  %2 = alloca i8**, align 8
  %byte = alloca i8, align 1
  %port = alloca i16, align 2
  store i32 %argc, i32* %1, align 4
  store i8** %argv, i8*** %2, align 8
  store i8 0, i8* %byte, align 1
  store i16 128, i16* %port, align 2
  %3 = load i8* %byte, align 1
  %4 = load i16* %port, align 2
  call void asm sideeffect "out %al, %dx", "{ax},{dx},~{dirflag},~{fpsr},~{flags}"(i8 %3, i16 %4) #1, !srcloc !1
  ret i32 0
}

attributes #0 = { nounwind uwtable "less-precise-fpmad"="false" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-nans-fp-math"="false" "stack-protector-buffer-size"="8" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #1 = { nounwind }

!llvm.ident = !{!0}

!0 = metadata !{metadata !"clang version 3.5.1 (tags/RELEASE_351/final)"}
!1 = metadata !{i32 115}
