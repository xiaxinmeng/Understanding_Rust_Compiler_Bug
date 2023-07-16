rust
#[repr(C)]
struct S {
  f1: u16,
  f2: u32,
}

let s = S { f1: 0, f2: 0 };
let ptr = &s as *const S;
let ptr = s.cast::<u16>().wrapping_offset(1); // now it points at the padding between the fields
let val = unsafe { *ptr }; // Is this line UB?
assert!(val == 0 || val > 0); // If no, is this line UB? If no, can the assertion fail?
