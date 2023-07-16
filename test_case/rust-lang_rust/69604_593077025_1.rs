rust
// Code
macro_rules! m { ($i: ident) => {
    struct S $i
}}

m!(i);

fn main() {}
