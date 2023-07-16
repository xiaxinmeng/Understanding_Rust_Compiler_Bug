
bug.rs:2:27: 2:33 error: constant evaluation error: unsupported constant expr [E0080]
bug.rs:2 pub const CONSTANT: Foo = Foo(1);
                                   ^~~~~~
bug.rs:2:27: 2:33 help: run `rustc --explain E0080` to see a detailed explanation
bug.rs:5:15: 5:23 note: for enum discriminant here
bug.rs:5     Variant = CONSTANT,
                       ^~~~~~~~
error: aborting due to previous error
