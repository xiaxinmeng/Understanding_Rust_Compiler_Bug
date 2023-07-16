
  %spec.select.i.i = select i1 %1, i64 0, i64 8
  br i1 %1, label %bb4.i1, label %bb4.i

[...]

  %spec.select.i16.i = select i1 %4, i64 0, i64 8
  br i1 %4, label %bb4.i1, label %bb11.i
