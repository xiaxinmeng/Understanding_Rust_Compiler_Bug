
bb0: {
  tmp0 = 5;
  tmp1 = 42;
  tmp2 = foo(tmp1) -> [return: bb1, unwind: bb2] 
}

bb1: {
  tmp3 = tmp0;
  ...
}

bb2: {
  resume;
}
