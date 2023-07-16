rust
let mut a = [1, 2, 3, 4];
let v: &mut u32 = u32::from_ne_bytes_mut(&mut a);
// Then potentially used with
let x = AtomicU32::from_mut(v);
