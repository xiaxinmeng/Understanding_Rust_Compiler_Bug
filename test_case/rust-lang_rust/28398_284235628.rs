llvm
; ModuleID = 'rust_out.cgu-0.rs'
source_filename = "rust_out.cgu-0.rs"
target datalayout = "e-m:e-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%MyEnum = type { i8, [3 x i8], [16384 x i32] }

; Function Attrs: norecurse nounwind uwtable
define void @_ZN8rust_out4slow17h69b71af2db2df8c0E(%MyEnum* nocapture dereferenceable(65540)) unnamed_addr #0 {
entry-block:
  %1 = getelementptr inbounds %MyEnum, %MyEnum* %0, i64 0, i32 0
  store i8 0, i8* %1, align 1
  %2 = getelementptr inbounds %MyEnum, %MyEnum* %0, i64 0, i32 2, i64 0
  store i32 32, i32* %2, align 4
  ret void
}

; Function Attrs: norecurse nounwind uwtable
define void @_ZN8rust_out4fast17hd831faf6e2383adeE(%MyEnum* nocapture dereferenceable(65540), i32) unnamed_addr #0 {
entry-block:
  %2 = getelementptr inbounds %MyEnum, %MyEnum* %0, i64 0, i32 0
  store i8 0, i8* %2, align 1
  %3 = getelementptr inbounds %MyEnum, %MyEnum* %0, i64 0, i32 2, i64 0
  store i32 %1, i32* %3, align 4
  ret void
}

attributes #0 = { norecurse nounwind uwtable }
