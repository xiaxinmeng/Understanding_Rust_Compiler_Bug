rust
// We have some bytes that encode the number 1 as a 32 bit integer in LE format, 
let le_bytes = [1, 0, 0, 0];

// Load the bytes into the `u32` verbatim
let x: u32 = *(&le_bytes as &u32);

// On a big endian machine x is now the number 16777216, so we convert 
// to big endian by, obviously, calling `to_le()`
x.to_le()
