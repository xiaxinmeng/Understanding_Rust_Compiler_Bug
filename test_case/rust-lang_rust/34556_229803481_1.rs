
bb0: {
  tmp3 = 5; // tmp0 optimized out
  tmp1 = 42;
  tmp2 = foo(tmp1) -> [return: bb1, unwind: bb2] 
}

bb1: {
  // tmp3 = tmp0 optimized out
  ...
}

bb2: {
  resume;
}
