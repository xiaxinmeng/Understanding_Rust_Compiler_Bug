llvm
...
  %17 = getelementptr inbounds i32, i32* %ptr.0148.i.i.i.i, i64 6
  %iter.sroa.0.1.i.i.i.i.6 = add nsw i32 %iter.sroa.0.0147.i.i.i.i, 7
  store i32 %iter.sroa.0.1.i.i.i.i.5, i32* %17, align 4, !noalias !4
  %18 = getelementptr inbounds i32, i32* %ptr.0148.i.i.i.i, i64 7
; Comparison using equality instruction:
  %exitcond.i.i.i.6 = icmp eq i32 %iter.sroa.0.1.i.i.i.i.6, 32767
  br i1 %exitcond.i.i.i.6, label %_ZN4core4iter8iterator8Iterator7collect17hc35826f1180bb746E.exit, label %bb35.i.i.i.i

_ZN4core4iter8iterator8Iterator7collect17hc35826f1180bb746E.exit: ; preds = %bb35.i.i.i.i
...
