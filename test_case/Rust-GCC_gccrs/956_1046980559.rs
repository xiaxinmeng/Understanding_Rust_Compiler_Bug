rust
macro_rules! add_exprs {
    ($($e:expr)+) => ( 0 $(+ $e) )
}

let a0 = add_exprs!(2); // expands to 0 + 2
let a1 = add_exprs!(a0); // expands to 0 + a0
