
struct S(u8, u8, u8);
let s = S { 0, 1, 2 }; // Parses, 0, 1 and 2 are valid fields, but unresolved identifier expressions
