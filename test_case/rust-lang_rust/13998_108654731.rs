
13998.rs:24:22: 24:30 error: mismatched types:
 expected `fn(&_)`,
    found `fn(_) {two_args}`
(expected concrete lifetime,
    found bound lifetime parameter ) [E0308]
13998.rs:24     let f = Bar::new(two_args);
                                 ^~~~~~~~
13998.rs:24:22: 24:30 help: run `rustc --explain E0308` to see a detailed explanation
