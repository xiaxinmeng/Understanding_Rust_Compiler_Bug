
  %f.sroa.0.0.ph.i = phi i8 [ 1, %foo-block ], [ 0, %entry-block ]
  %trunc.i.i.i = trunc i8 %f.sroa.0.0.ph.i to i2
  %switch.i = icmp eq i2 %trunc.i.i.i, 0
...
x-block:
  br i1 %switch.i, label %x2-block, label %x1-block
...
y-block:
  %cond.i.i.i = icmp eq i8 %f.sroa.0.0.ph.i, 0
  br i1 %cond.i.i.i, label %y1-block, label %y2-block
