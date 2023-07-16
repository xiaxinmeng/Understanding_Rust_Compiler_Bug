rust

// 2015 crate
macro_rules! foo {
  ($x:ident) => (fn $x() {})
}

// 2018 crate
foo!(async); // what happens?
