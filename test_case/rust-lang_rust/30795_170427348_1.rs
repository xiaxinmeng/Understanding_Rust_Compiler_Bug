 llvm
; ModuleID = 'foo.0.rs'
target datalayout = "e-m:o-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-darwin"

@errno = external global i32

; Function Attrs: uwtable
define i32 @_ZN3foo20h5ca9ce4ccbe13eabhaaE() unnamed_addr #0 {
entry-block:
  %0 = load i32, i32* @errno, align 4
  ret i32 %0
}

attributes #0 = { uwtable "no-frame-pointer-elim"="true" }
