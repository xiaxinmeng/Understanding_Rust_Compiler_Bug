rust
let arr = [0;32];
let slice = &arr[0..8];
slice.get_unchecked(16);
