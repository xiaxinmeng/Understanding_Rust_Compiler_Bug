rust
#![feature(try_blocks)] // require nightly
let tuple: (Option<A>, Option<B>) = ...;
let res: Option<(A, B)> = try { (tuple.0?, tuple.1?) }
