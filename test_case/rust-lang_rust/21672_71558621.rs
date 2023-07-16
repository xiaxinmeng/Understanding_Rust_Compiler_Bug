 rust
let _: Vec<u8> = (0..10).collect();
//~^ error: type mismatch resolving `<core::ops::Range<i32> as core::iter::Iterator>::Item == u8`
let _: Vec<u8> = range(0, 10).collect();  // OK
