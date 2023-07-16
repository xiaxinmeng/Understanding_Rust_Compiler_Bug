
foo.rs:2:49: 2:54 error: binary operation `+` cannot be applied to type `&&i8`
foo.rs:2   println!("{}", vec![1i8,2,3].iter().max_by(|n|n + 0).map(|&e|e))
