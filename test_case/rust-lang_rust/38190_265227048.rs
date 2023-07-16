rust
macro_rules! m { ($i:item) => { mod x { $i } } }

m! { mod y; } // error: no file y.rs or y/mod.rs
