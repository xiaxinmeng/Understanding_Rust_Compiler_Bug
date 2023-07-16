rust
macro_rules! as_item { ($i:item) => { $i } }
macro_rules! m { ($($t:tt)*) => { as_item!(mod x { $($t)* }); } }

m! { mod y; } // error: no file x/y.rs or x/y/mod.rs
