 Rust
(x, y) = (1, 2); // you cannot do this (at least not today; there may be a postponed RFC asking for it)
// -> it seems logical since you have to declare it before
let (x, y) = (1, 2); // but you can do this
// -> which is done here

let x.foo = 3; // you cannot do this (and almost certainly never will be able to)
// -> you can't allocate just a sub element of a struct without allocating the struct
x.foo = 3; // but you can do this
// -> it's just assigning a value to a sub element of an allocated struct
