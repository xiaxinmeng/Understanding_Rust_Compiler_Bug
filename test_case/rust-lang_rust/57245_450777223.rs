rust
let iter = || (1..=10).map(|x| println!("{}", x));
iter().count(); // prints 1-10
iter().rev().count(); // prints 10-1
iter().next(); // prints 1
iter().rev().last(); // prints 10-1
