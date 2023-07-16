rust
let _temp1 = a;
// _temp2 created before `_x`, so it's dropped after
let _temp2 = b;

let (_x, ) = a;
let (, _y) = b;
