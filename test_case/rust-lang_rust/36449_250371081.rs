 rust
let res: Result<u32, Void> = ...;
// valid
let Ok(x) = res;
// ERROR!
let Ok(x) = res.as_ref();

enum Three { A, B, C }
enum Two { A, B }
enum One { A }
enum Zero { }

match &some_three {
    &A => ...
    &B => ...
    &C => ...
}

match &some_two {
    &A => ...
    &B => ...
}

match &some_one {
    &A => ...
}

// ERROR!
match &some_zero {
}
