
  %f.sroa.0.0.ph.i = phi i8 [ 1, %foo-block ], [ 0, %entry-block ]
  %switch = icmp eq i8 %f.sroa.0.0.ph.i, 1
...
x-block:
  br i1 %switch, label %x1-block, label %x2-block
...
y-block:
  %cond.i.i.i = icmp eq i8 %f.sroa.0.0.ph.i, 0
  br i1 %cond.i.i.i, label %y1-block, label %y2-block
