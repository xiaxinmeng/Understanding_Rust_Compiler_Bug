
$ rustc --crate-type=lib --emit llvm-ir -O inf.rs            
inf.rs:1:1: 3:2 warning: function cannot return without recurring, #[warn(unconditional_recursion)] on by default
inf.rs:1 pub fn f(x: i32) -> i32 {
inf.rs:2     f(x)
inf.rs:3 }
inf.rs:2:5: 2:9 note: recursive call site
inf.rs:2     f(x)
             ^~~~
inf.rs:1:1: 3:2 help: a `loop` may express intention better if this is on purpose
