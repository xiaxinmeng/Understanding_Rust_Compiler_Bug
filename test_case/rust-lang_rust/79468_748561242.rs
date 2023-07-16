rust
const HUGE_SIZE: usize = !0usize / 8;
static MY_TOO_BIG_ARRAY_2: [u8; HUGE_SIZE] = [0x00; HUGE_SIZE];
