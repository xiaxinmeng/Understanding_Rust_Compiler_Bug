rust
let helper = || -> impl Debug { 5_i32 };
let x = helper();
let y = x + 1_i32; // ERROR
