
issue28308.rs:3:45: 3:50 error: cannot apply unary operator `!` to type `&'static str`
issue28308.rs:3 macro_rules! my_assert2 { ($e:expr) => { if !($e) { panic!() } } }
                                                            ^~~~~
issue28308.rs:7:4: 7:24 note: in this expansion of my_assert2! (defined in issue28308.rs)
