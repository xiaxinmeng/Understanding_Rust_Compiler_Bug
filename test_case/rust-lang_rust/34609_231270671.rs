
let iref = &0;
let p = iref as *const u8;
// error: casting `&usize` as `*const u8` is invalid
