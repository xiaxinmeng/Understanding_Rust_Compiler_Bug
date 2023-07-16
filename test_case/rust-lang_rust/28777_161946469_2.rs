
main.rs:2:26: 2:27 error: mismatched types:
 expected `()`,
    found `_`
(expected (),
    found integral variable) [E0308]
main.rs:2     let v1 = { if 1 > 2 {2} else {3}  5 };
                                   ^
main.rs:2:26: 2:27 help: run `rustc --explain E0308` to see a detailed explanation
main.rs:2:35: 2:36 error: mismatched types:
 expected `()`,
    found `_`
(expected (),
    found integral variable) [E0308]
main.rs:2     let v1 = { if 1 > 2 {2} else {3}  5 };
                                            ^
main.rs:2:35: 2:36 help: run `rustc --explain E0308` to see a detailed explanation
error: aborting due to 2 previous errors
