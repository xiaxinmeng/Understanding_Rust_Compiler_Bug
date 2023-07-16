assembly
start:
  %_4 = icmp eq i32 %q, 5
  %. = select i1 %_4, i8 1, i8 2
  store i8 %., i8* %a, align 1
  %_6 = zext i8 %. to i64
  switch i64 %_6, label %bb5 [
    i64 0, label %bb4
    i64 1, label %bb8
    i64 2, label %bb7
  ]
