
$ rustc --crate-type lib b17021.rs 
b17021.rs:3:1: 5:2 warning: code is never used: `bug`, #[warn(dead_code)] on by default
b17021.rs:3 fn bug() {
b17021.rs:4   let c = box |&:| {};
b17021.rs:5 }
b17021.rs:4:7: 4:8 warning: unused variable: `c`, #[warn(unused_variable)] on by default
b17021.rs:4   let c = box |&:| {};
                  ^
