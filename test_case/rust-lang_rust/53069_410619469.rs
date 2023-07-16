rust
// sse2 function
if has_sse3 {  // detect sse3 at run-time
    // executing this on sse2 hardware is UB
    // so this cannot be speculatively executed, re-ordered out of the if, etc. 
    sse3_intrinsics(); 
}
