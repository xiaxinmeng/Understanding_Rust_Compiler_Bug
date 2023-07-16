
; ModuleID = 'a.0.rs'
target datalayout = "e-m:e-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"2.std::slice::Iter<u8>" = type { i8*, i8*, %"2.std::marker::PhantomData<&'static u8>" }
%"2.std::marker::PhantomData<&'static u8>" = type {}

; Function Attrs: norecurse readonly uwtable
define i64 @_ZN1a9slice_len17hc2890fcf572cc394E(%"2.std::slice::Iter<u8>"* noalias nocapture readonly dereferenceable(16)) unnamed_addr #0 {
entry-block:
  %1 = getelementptr inbounds %"2.std::slice::Iter<u8>", %"2.std::slice::Iter<u8>"* %0, i64 0, i32 1
  %2 = bitcast i8** %1 to i64*
  %3 = load i64, i64* %2, align 8, !alias.scope !0, !noalias !5
  %4 = bitcast %"2.std::slice::Iter<u8>"* %0 to i64*
  %5 = load i64, i64* %4, align 8, !alias.scope !0, !noalias !5
  %6 = sub i64 %3, %5
  ret i64 %6
}

; Function Attrs: norecurse nounwind readonly uwtable
define i64 @_ZN1a10slice_len217h415029022c569338E(%"2.std::slice::Iter<u8>"* noalias nocapture readonly dereferenceable(16)) unnamed_addr #1 {
entry-block:
  %1 = getelementptr inbounds %"2.std::slice::Iter<u8>", %"2.std::slice::Iter<u8>"* %0, i64 0, i32 1
  %2 = bitcast i8** %1 to i64*
  %3 = load i64, i64* %2, align 8, !alias.scope !7, !noalias !10
  %4 = bitcast %"2.std::slice::Iter<u8>"* %0 to i64*
  %5 = load i64, i64* %4, align 8, !alias.scope !7, !noalias !10
  %6 = sub i64 %3, %5
  ret i64 %6
}

attributes #0 = { norecurse readonly uwtable }
attributes #1 = { norecurse nounwind readonly uwtable }

!0 = !{!1, !3}
!1 = distinct !{!1, !2, !"_ZN79_$LT$std..slice..Iter$LT$$u27$a$C$$u20$T$GT$$u20$as$u20$std..iter..Iterator$GT$9size_hint17h39fecfdf3007b42aE: argument 1"}
!2 = distinct !{!2, !"_ZN79_$LT$std..slice..Iter$LT$$u27$a$C$$u20$T$GT$$u20$as$u20$std..iter..Iterator$GT$9size_hint17h39fecfdf3007b42aE"}
!3 = distinct !{!3, !4, !"_ZN4core4iter6traits17ExactSizeIterator3len17hb31ac8a36a76a083E: argument 0"}
!4 = distinct !{!4, !"_ZN4core4iter6traits17ExactSizeIterator3len17hb31ac8a36a76a083E"}
!5 = !{!6}
!6 = distinct !{!6, !2, !"_ZN79_$LT$std..slice..Iter$LT$$u27$a$C$$u20$T$GT$$u20$as$u20$std..iter..Iterator$GT$9size_hint17h39fecfdf3007b42aE: argument 0"}
!7 = !{!8}
!8 = distinct !{!8, !9, !"_ZN79_$LT$std..slice..Iter$LT$$u27$a$C$$u20$T$GT$$u20$as$u20$std..iter..Iterator$GT$9size_hint17h39fecfdf3007b42aE: argument 1"}
!9 = distinct !{!9, !"_ZN79_$LT$std..slice..Iter$LT$$u27$a$C$$u20$T$GT$$u20$as$u20$std..iter..Iterator$GT$9size_hint17h39fecfdf3007b42aE"}
!10 = !{!11}
!11 = distinct !{!11, !9, !"_ZN79_$LT$std..slice..Iter$LT$$u27$a$C$$u20$T$GT$$u20$as$u20$std..iter..Iterator$GT$9size_hint17h39fecfdf3007b42aE: argument 0"}
