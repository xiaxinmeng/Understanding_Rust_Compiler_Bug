 rust
let i: u64 = {larger than array bounds, representable within 32 bits};
let j: u64 = {too large to represent within 32 bits};
let x = [0; ...];
let a = x[i]; // slice out-of-bounds panic on any platform
let b = x[j]; // slice out-of-bounds panic on 64-bit, integer truncation on 32-bit
