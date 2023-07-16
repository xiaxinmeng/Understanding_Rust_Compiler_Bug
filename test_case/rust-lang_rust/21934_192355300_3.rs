
hello.rs:9:18: 9:24 error: mismatched types:
 expected `i32`,
    found `A::Foo`
(expected i32,
    found struct `A::Foo`) [E0308]
hello.rs:9     let b: i32 = A::Bar;
                            ^~~~~~
hello.rs:9:18: 9:24 help: run `rustc --explain E0308` to see a detailed explanation
error: aborting due to previous error
