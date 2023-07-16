 llvm
; Function Attrs: uwtable
define internal void @_ZN10into_inner20h17287350f8168d4atbaE(i8) unnamed_addr #0 {
entry-block:
  %dropflag_hint_83 = alloca i8
  %dropflag_hint_91 = alloca i8
  %x = alloca %Drp
  %d = alloca %Drp
  %temp0 = alloca {}
  %temp1 = alloca %Drp
  %temp2 = alloca {}
  %arg0 = alloca %Drp
  store i8 61, i8* %dropflag_hint_83
  store i8 61, i8* %dropflag_hint_91
  %1 = bitcast %Drp* %arg0 to i8*
  store i8 %0, i8* %1, align 1
  br label %"BB(0)"

"BB(0)":                                          ; preds = %entry-block
  %2 = bitcast %Drp* %arg0 to i8*
  %3 = load i8, i8* %2, align 1
  %4 = bitcast %Drp* %x to i8*
  store i8 %3, i8* %4, align 1
  br label %"BB(3)"

"BB(1)":                                          ; preds = %"BB(4)"
  ret void

"BB(2)":                                          ; No predecessors!
  unreachable

"BB(3)":                                          ; preds = %"BB(0)"
  %5 = bitcast %Drp* %x to i8*
  %6 = load i8, i8* %5, align 1
  %7 = bitcast %Drp* %temp1 to i8*
  store i8 %6, i8* %7, align 1
  %8 = bitcast %Drp* %temp1 to i8*
  %9 = load i8, i8* %8, align 1
  %10 = bitcast %Drp* %d to i8*
  store i8 %9, i8* %10, align 1
  call void @_ZN3Drp9drop.332317hb0d710f73d73278eE(%Drp* %temp1)
  call void @_ZN3Drp9drop.332317hb0d710f73d73278eE(%Drp* %d)
  br label %"BB(4)"

"BB(4)":                                          ; preds = %"BB(3)"
  call void @_ZN3Drp9drop.332317hb0d710f73d73278eE(%Drp* %x)
  br label %"BB(1)"
}
