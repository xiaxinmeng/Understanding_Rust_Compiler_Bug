rust
const EMPTY_SLICE: &[u8] = &[0; 0];
...
let mut bufs = [EMPTY_SLICE; IOVEC_MAX];
