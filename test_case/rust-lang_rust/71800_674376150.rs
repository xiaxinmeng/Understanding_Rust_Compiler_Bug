rust
const ERR: i32 = 1/0;

fn foo() { if false { let _val = ERR; } }
