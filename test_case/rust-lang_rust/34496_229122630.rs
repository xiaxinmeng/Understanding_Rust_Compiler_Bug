
let x = &mut something;
let y = unsafe { transmute_copy(&x) };
// Use both *x and *y
