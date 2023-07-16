rust
let mut array: [MaybeUninit<i32>; 3] = MaybeUninit::uninit_array();
array[0].write(0);
array[1].write(1);
array[2].write(2);
