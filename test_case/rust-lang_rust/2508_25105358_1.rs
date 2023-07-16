
$ rustc 2508.rs 
2508.rs:3:4: 5:5 error: type `~str` does not implement any method in scope named `foo`
2508.rs:3     do s.foo {
2508.rs:4         // nothing
2508.rs:5     }

