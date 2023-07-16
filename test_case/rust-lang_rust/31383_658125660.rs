rust
macro_rules! A {() => {"a"}}
const A: &'static str = A!();
macro_rules! B {() => {"b"}}
const B: &'static str = B!();
const AB: &'static str = concat!(A!(), B!());
