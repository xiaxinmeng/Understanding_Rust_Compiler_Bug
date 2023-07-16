rust
let mut b: u8 = uninitialized();
let bref = &mut b; // Insta UB ? 
