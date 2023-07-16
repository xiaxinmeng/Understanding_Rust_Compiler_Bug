
  %iv = phi i64 [ %iv.inc, %bb11 ], [ 0, %bb9.preheader ]
  -->  {0,+,1}<nuw><%bb9> U: [0,-1) S: [0,-1)		Exits: ((-1 + (%dest.1 umin %src.1)) umin %dest.1 umin %src.1)		LoopDispositions: { %bb9: Computable }
  %iv.inc = add nuw i64 %iv, 1
  -->  {1,+,1}<nuw><%bb9> U: [1,0) S: [1,0)		Exits: (1 + ((-1 + (%dest.1 umin %src.1)) umin %dest.1 umin %src.1))		LoopDispositions: { %bb9: Computable }
...
  exit count for bb9: %src.1
  exit count for bb10: %dest.1
  exit count for bb11: (-1 + (%dest.1 umin %src.1))
