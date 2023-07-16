 text
hello.rs:5:1: 47:2 error: not all control paths return a value [E0269]
hello.rs: 5 fn foo() -> i32 {
hello.rs: 6     let x = 5;
hello.rs: 7     match x {
hello.rs: 8         1 => {
hello.rs: 9             1
hello.rs:10         },
            ...
hello.rs:5:1: 47:2 help: run `rustc --explain E0269` to see a detailed explanation
error: aborting due to previous error
