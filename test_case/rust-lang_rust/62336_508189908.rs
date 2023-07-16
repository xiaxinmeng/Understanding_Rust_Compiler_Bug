rust
struct NoCopy;
let x = [NoCopy; 0]; // OK.
let x = [NoCopy; 1]; // OK.
let x = [NoCopy; 2]; // ERROR.
