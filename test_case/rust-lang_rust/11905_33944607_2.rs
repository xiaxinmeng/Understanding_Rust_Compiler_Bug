
let mut i = 0;
try_finally(
    &mut i, 
    |i| { ... *i += 1; ... },
    |i| { ... *i ... });
