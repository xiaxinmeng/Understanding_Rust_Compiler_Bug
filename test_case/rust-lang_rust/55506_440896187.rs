rust
let c : impl Fn(i32) = ... ;
c(); // `c` must be used. If `c` is discarded, it probably indicates something wrong.

let r = c(); // I think this attribute does not care `r` is used or not
