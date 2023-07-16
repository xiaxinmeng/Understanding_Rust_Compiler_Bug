 shell
$ rustc main.rs
main.rs:5:16: 5:17 error: cannot refer to other statics by value, use the address-of operator or a constant instead
main.rs:5 static B: i8 = A;
                         ^
error: aborting due to previous error
