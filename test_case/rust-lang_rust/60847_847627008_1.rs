rust
let mut arr = [1u32, 2];
let v = arr.as_mut_ptr();
test(v, unsafe { &mut *v });
