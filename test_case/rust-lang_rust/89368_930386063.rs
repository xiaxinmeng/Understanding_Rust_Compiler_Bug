rust
let ary: [u8; 80] = [0u8; 80];
// let ary2 = ary; // <- this would get a new address
let slice: &[u8] = ary.as_slice(); // <- this address won't change for the rest of the method
