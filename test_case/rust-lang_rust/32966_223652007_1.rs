 rust
tmp0 = &mut var0
tmp1 = None
tmp2 = &mut tmp1
tmp3 = tmp0 as *const T
tmp4 = *tmp3
tmp5 = tmp2 as *const T
tmp6 = tmp0 as *mut T
*tmp6 = *tmp5
tmp7 = tmp2 as *mut T
*tmp7 = tmp4
var1 = tmp1
