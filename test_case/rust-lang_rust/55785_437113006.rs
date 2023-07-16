rust
let val: impl ?Sized = ...;

let dest = alloc(Layout::for_value(&val));
ptr::copy_nonoverlapping(&val as *const _ as *const u8, dest, mem::size_of_val(&val));

mem::forget(val); // We need this!
