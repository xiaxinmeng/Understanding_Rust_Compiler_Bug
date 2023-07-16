
; ModuleID = 'segfault.0.rs'
target datalayout = "e-m:e-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"9.my_crate::Foo" = type { {} }

; Function Attrs: uwtable
define noalias %"9.my_crate::Foo"* @_ZN7box_foo20hf984887980016f2cgaaE() unnamed_addr #0 {
entry-block:
  %0 = tail call %"9.my_crate::Foo" @"_ZN13_$LT$impl$GT$3new20h7a0183b89d396887kaaE"()
  ret %"9.my_crate::Foo"* inttoptr (i64 1 to %"9.my_crate::Foo"*)
}

declare %"9.my_crate::Foo" @"_ZN13_$LT$impl$GT$3new20h7a0183b89d396887kaaE"() unnamed_addr

attributes #0 = { uwtable }
