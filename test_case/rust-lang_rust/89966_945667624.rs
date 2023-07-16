rust
let a = &mut 5;
drop(a);
*a += 1; // error
