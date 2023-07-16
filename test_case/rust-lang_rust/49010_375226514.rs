llvm
  %min.0.ph32.i.i.i = phi i64* [ null, %bb8.lr.ph.i.lr.ph.lr.ph.i.i.i ], [ %.lcssa23.i164.pre-phi.i.pre-phi.i.pre-phi, %bb11.i165.i.i ]
  ; ...
  %328 = icmp ne i64* %min.0.ph32.i.i.i, null
  ; a while later
  call void @llvm.assume(i1 %328) #13, !noalias !213
