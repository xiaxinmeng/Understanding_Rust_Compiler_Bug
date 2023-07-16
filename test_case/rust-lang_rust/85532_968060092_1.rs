rs
// Here `memory.get_byte(some_address)` is used to abstract the architecture details.
let src: u8 = memory.get_byte(some_address);
let dst: u8 = memory.get_byte(some_address);

// Here is where the signed functions are useful, they computes the (signed) overflow bit.
// `codes` is the variable containing the conditions codes described upper.
let (result, v) = (src as i8).carrying_add(dst, codes.x);
// Use the unsigned big int function to compute the carry (unsigned overflow) bit.
let (_, c) = src.carrying_add(dst, codes.x);

// Now computes the condition codes using the result value and the 
codes.x = c;
codes.v = v;
codes.c = c;
// Other codes [...]

// Write back the result
memory.set_byte(some_address, result);
