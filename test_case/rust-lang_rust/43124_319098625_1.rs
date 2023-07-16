llvm
...
  store i16 %iter.sroa.0.0.152.i.i.i.i, i16* %ptr.0150.i.i.i.i, align 2, !noalias !4
  %12 = getelementptr inbounds i16, i16* %ptr.0150.i.i.i.i, i64 1
  %13 = add i64 %local_len.sroa.5.0149.i.i.i.i, 1
; comparison using unsigned less than instruction:
  %14 = icmp ult i16 %.iter.sroa.0.0151.i.i.i.i, 32767
  %15 = zext i1 %14 to i16
  %.iter.sroa.0.0.i.i.i.i = add i16 %15, %.iter.sroa.0.0151.i.i.i.i
  br i1 %14, label %bb35.i.i.i.i, label %_ZN4core4iter8iterator8Iterator7collect17h730733b2264e9b53E.exit
...
