 llvm
; I was hoping this (the double `!rust` bit) worked
; <hello::Baz as hello::Bar<bool>>::bar
define i1 @_(%Baz*) !rust !1 !rust !2 {
  ; ..
}

!1 = !{!"fn", !"fn(&Baz) -> bool"} ; function pointer call
!2 = !{!"dyn", !"Bar<bool>", !"bar"} ; dynamic dispatch

; but it didn't so instead I'm doing this
; <hello::Baz as hello::Bar<bool>>::bar
define i1 @_(%Baz*) !rust !3 {
  ; ..
}

; the two nodes get mixed into one
!3 = !{!"fn", !"fn(&Baz) -> bool", !"dyn", !"Bar<bool>", !"bar"}
