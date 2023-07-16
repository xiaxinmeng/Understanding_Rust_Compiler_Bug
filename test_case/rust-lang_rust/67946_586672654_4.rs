llvm
...
; Function Attrs: nonlazybind uwtable
define void @bar(i1 zeroext %b) unnamed_addr #0 {
start:
  ret void
}

; foo::foo
; Function Attrs: nonlazybind uwtable
define void @_ZN3foo3foo17he8a114b84f989db4E() unnamed_addr #0 {
start:
  call void bitcast (void (i1)* @bar to void ()*)()
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}
...
