rust
let mut value = 0;
let value_ptr = &mut value as *mut i32;
let vec = vec![value_ptr];
vec.as_ptr().read().write(100);
assert_eq!(value, 100);
