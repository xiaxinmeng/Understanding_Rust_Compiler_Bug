
; ModuleID = 'style_stable.0.rs'
target datalayout = "e-m:e-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"11.style::properties::PropertyDeclaration" = type { i64, [0 x i64], [60 x i64] }

; Function Attrs: norecurse nounwind readonly uwtable
define i64 @_ZN12style_stable10with_match17h5ee51730fa1d4ed4E(%"11.style::properties::PropertyDeclaration"* noalias nocapture readonly dereferenceable(488)) unnamed_addr #0 {
entry-block:
  %.idx = getelementptr %"11.style::properties::PropertyDeclaration", %"11.style::properties::PropertyDeclaration"* %0, i64 0, i32 0
  %.idx.val = load i64, i64* %.idx, align 8
  ret i64 %.idx.val
}

; Function Attrs: norecurse nounwind readonly uwtable
define i64 @_ZN12style_stable14with_intrinsic17h38553922905ee296E(%"11.style::properties::PropertyDeclaration"* noalias nocapture readonly dereferenceable(488)) unnamed_addr #0 {
entry-block:
  %1 = getelementptr inbounds %"11.style::properties::PropertyDeclaration", %"11.style::properties::PropertyDeclaration"* %0, i64 0, i32 0
  %2 = load i64, i64* %1, align 8, !range !0
  ret i64 %2
}

attributes #0 = { norecurse nounwind readonly uwtable }

!0 = !{i64 0, i64 129}
