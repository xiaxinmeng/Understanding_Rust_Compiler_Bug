rust
let mut p = [MaybeUninit::<u8>::uninit(); 571]; // memset
let mut p: [MaybeUninit<u8>; 571] = MaybeUninit::uninit_array(); // no memset
let mut p = MaybeUninit::<[u8; 571]>::uninit(); // no memset
