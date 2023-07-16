rust
let mut array: [MaybeUninit<i32>; 3] = MaybeUninit::uninit_array();
array[0] = MaybeUninit::new(0);
array[1] = MaybeUninit::new(1);
array[2] = MaybeUninit::new(2);
