
$ rustc main.rs
main.rs:5:16: 5:17 error: cannot refer to other statics by value, use the address-of operator or a constant instead
main.rs:5 const X: i32 = x;
                         ^
main.rs:5:16: 5:17 error: constants cannot refer to other statics, insert an intermediate constant instead
main.rs:5 const X: i32 = x;
                         ^
