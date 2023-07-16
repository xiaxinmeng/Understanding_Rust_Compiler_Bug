
issue28308.rs:2:45: 7:22 error: cannot apply unary operator `!` to type `&'static str`
issue28308.rs:2 macro_rules! my_assert1 { ($e:expr) => { if !$e { panic!() } } }
issue28308.rs:3 macro_rules! my_assert2 { ($e:expr) => { if !($e) { panic!() } } }
issue28308.rs:4 // Meaningless text goes here
issue28308.rs:5 // And some more meaningless text
issue28308.rs:6 fn main() {
issue28308.rs:7    my_assert1!("test");
issue28308.rs:7:4: 7:24 note: in this expansion of my_assert1! (defined in issue28308.rs)
