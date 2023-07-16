assembly
start:
  %_4 = icmp eq i32 %q, 5
  %. = select i1 %_4, i8 1, i8 2
  store i8 %., i8* %a, align 1
  %spec.select = select i1 %_4, i32 1, i32 2
  ret i32 %spec.select
