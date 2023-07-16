rust
// We have some bytes that encode the number 1 as a 32 bit integer in LE format, 
let le_bytes = [1, 0, 0, 0];

// So much nicer!
let x = u32::from_le_bytes(le_bytes);
