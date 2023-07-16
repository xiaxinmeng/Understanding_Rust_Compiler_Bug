rust

// 2015 crate
macro_rules! foo {
  () => (fn async() {})
}

// 2018 crate
foo!(); // what happens?
