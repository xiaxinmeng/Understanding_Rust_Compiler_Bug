
error: unnecessary parentheses around function argument
    --> src\librustc_apfloat\ieee.rs:1437:56
     |
1437 |         let exp_change = cmp::min(cmp::max(exp as i32, (-max_change - 1)), max_change);
     |                                                        ^^^^^^^^^^^^^^^^^ help: remove these parentheses
     |
note: lint level defined here
    --> src\librustc_apfloat\lib.rs:45:9
     |
45   | #![deny(warnings)]
     |         ^^^^^^^^
     = note: #[deny(unused_parens)] implied by #[deny(warnings)]
   Compiling rustc_binaryen v0.0.0 (file:///C:/projects/rust/src/librustc_binaryen)
error: aborting due to previous error
error: Could not compile `rustc_apfloat`.
