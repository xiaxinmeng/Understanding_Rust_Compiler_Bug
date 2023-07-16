 rust
macro_rules! foo ( ($t: ty) => { fn foo<'a>(a: &'a int, b: $t) { ... } })

foo!(&'a int)
