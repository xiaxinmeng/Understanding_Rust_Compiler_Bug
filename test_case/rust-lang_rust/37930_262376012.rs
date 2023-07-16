
define i8 @_start(i8* %condition) {
entry-block:
  br label %bb1.preheader.i

bb1.preheader.i:
  %f.sroa.0.0.ph.i = phi i8 [ 1, %bb4.i.i.i.i ], [ 0, %entry-block ]
  %switch.i = icmp eq i8 %f.sroa.0.0.ph.i, 1
  br label %loop

loop:
  br i1 %switch.i, label %ret1, label %i101

i101:
  %0 = load volatile i8, i8* %condition
  %1 = icmp eq i8 %0, 0
  br i1 %1, label %bb4.i.i.i.i, label %loop

bb4.i.i.i.i:
  %cond.i.i.i = icmp eq i8 %f.sroa.0.0.ph.i, 0
  br i1 %cond.i.i.i, label %bb1.preheader.i, label %ret0

ret1:
  ret i8 1

ret0:
  ret i8 0
}
 