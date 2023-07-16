
bb0: {
    var0 = const 4i32; 
    tmp1 = &mut var0; // and now finds there was a reference taken (which later gets cast into aliased raw pointer)
    tmp0 = &mut (*tmp1);
    var1 = tmp0 as *mut i32 (Misc); 
    var0 = const 5i32; // looks like a plain unused assignment, so gets removed
    ... // var0 is never used again in any path other than through the pointer in var1
}
