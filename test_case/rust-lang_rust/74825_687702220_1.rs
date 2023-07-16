rust
use std::io::{IoSliceMut, Read};

// Capn' Proto recommends framing in 4 bytes
let reader = vec![0_u8, 0, 0, 0, 1, 2, 3, 4];
let mut buf = vec![0; 8];
let mut iovecs = buf
    .chunks_exact_mut(4)
    .map(IoSliceMut::new)
    .collect::<Vec<_>>();
assert_eq!(reader.as_slice().read_vectored(&mut iovecs).unwrap(), 8);
assert_eq!(buf, reader);
