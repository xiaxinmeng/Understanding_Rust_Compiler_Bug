llvm
define <16 x i8> @test(<16 x i8> %x, <16 x i8> %y) #0 {
start:
  %mask = call <16 x i8> @llvm.x86.sse2.pcmpgt.b(<16 x i8> %x, <16 x i8> %y)
  %elem = extractelement <16 x i8> %mask, i8 0
  %cmp = icmp eq i8 %elem, 255
  br i1 %cmp, label %end, label %loop

loop:
  %mask2 = phi <16 x i8> [ %mask3, %loop ], [ %mask, %start ]
  %x2 = phi <16 x i8> [ %x3, %loop ], [ %x, %start ]
  %x3 = call <16 x i8> @llvm.x86.sse41.pblendvb(<16 x i8> %mask2, <16 x i8> %x2, <16 x i8> %y)
  %mask3 = call <16 x i8> @llvm.x86.sse2.pcmpgt.b(<16 x i8> %x3, <16 x i8> %y)
  %elem2 = extractelement <16 x i8> %mask3, i8 0
  %cmp2 = icmp eq i8 %elem2, 255
  br i1 %cmp2, label %end, label %loop

end:
  %x4 = phi <16 x i8> [ %x3, %loop ], [ %x, %start ]
  ret <16 x i8> %x4
}
