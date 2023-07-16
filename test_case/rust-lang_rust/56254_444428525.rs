rust
let w = &2phase *v;
let vraw = v as *mut _; // let's assume this wouldn't reborrow, but just cast
let wraw = w as *mut _; // let's assume this wouldn't reborrow, but just cast
// Now this is okay, as both pointers carry the same tag
*vraw = 4;
*wraw = 5;
*vraw = 6;
*wraw = 7;
