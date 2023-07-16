
rustup run nightly rustc main.rs
main.rs:11:6: 6:19 error: mismatched types:
 expected `&'static Bar`,
    found `Bar`
(expected &-ptr,
    found struct `Bar`) [E0308]
(internal compiler error: unprintable span)
main.rs:11:1: 11:11 note: in this expansion of foo! (defined in main.rs)
main.rs:11:6: 6:19 help: run `rustc --explain E0308` to see a detailed explanation
