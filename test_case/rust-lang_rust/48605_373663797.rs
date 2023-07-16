rust
let x = |mut y: isize| 10; // correctly flags mut y, but also contains a spurious one that spans the entire RHS of the statement

let x = |mut y: isize| y = 32; // similar to the above, flags entire RHS, but does not flag mut y
