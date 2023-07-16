rust
use std::io::*;
let res: Result<()> = (0..10)
    .map(|x| writeln!(stdout(), "{}", x))
    .collect();
assert!(res.is_ok());
