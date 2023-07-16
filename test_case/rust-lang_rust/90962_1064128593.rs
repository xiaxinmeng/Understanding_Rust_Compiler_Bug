rust
static A: [u8; 2] = [0, 0];

assert!(
    addr_of!(A[0]).align_offset(2) == 0
    || addr_of!(A[1]).align_offset(2) == 0
);
