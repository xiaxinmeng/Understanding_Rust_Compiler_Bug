
<anon>:8:22: 8:23 error: the trait bound `Y: std::fmt::Debug` is not satisfied [E0277]
<anon>:8     println!("{:?}", x);
                              ^
<anon>:8:22: 8:23 help: see the detailed explanation for E0277
<anon>:8:22: 8:23 note: `Y` cannot be formatted using `:?`; if it is defined in your crate, add `#[derive(Debug)]` or manually implement it
