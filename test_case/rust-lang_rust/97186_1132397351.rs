
; inside repro::System<FP,_>::poll:
; ...
  call void @"_ZN63_$LT$repro..SomeFoo$LT$_$GT$$u20$as$u20$repro..Foo$LT$_$GT$$GT$3foo17he0a8b10f80c417d4E"
; ...

; <repro::SomeFoo<_> as repro::Foo<_>>::foo
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN63_$LT$repro..SomeFoo$LT$_$GT$$u20$as$u20$repro..Foo$LT$_$GT$$GT$3foo17h0993b7b03477db6dE"(%"SomeFoo<4_usize>"* align 1 %self) unnamed_addr #1 {
start:
  ret void
}

; <repro::SomeFoo<_> as repro::Foo<_>>::foo
; Function Attrs: nonlazybind uwtable
declare hidden void @"_ZN63_$LT$repro..SomeFoo$LT$_$GT$$u20$as$u20$repro..Foo$LT$_$GT$$GT$3foo17he0a8b10f80c417d4E"(%"SomeFoo<4_usize>"* align 1) unnamed_addr #1
