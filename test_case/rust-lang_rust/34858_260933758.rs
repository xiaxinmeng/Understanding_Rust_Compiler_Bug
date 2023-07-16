
src/main.rs:1:1: 6:2 warning: function cannot return without recurring, #[warn(unconditional_recursion)] on by default
src/main.rs:1 fn foo<F, X>(cb: F) -> X
src/main.rs:2     where F : for<'a> FnOnce(&'a u64) -> X {
src/main.rs:3     foo(move |expr1| {
src/main.rs:4         cb(expr1)
src/main.rs:5     })
src/main.rs:6 }
src/main.rs:3:5: 5:7 note: recursive call site
src/main.rs:3     foo(move |expr1| {
src/main.rs:4         cb(expr1)
src/main.rs:5     })
src/main.rs:1:1: 6:2 help: a `loop` may express intention better if this is on purpose
src/main.rs:1:1: 6:2 error: reached the recursion limit during monomorphization
src/main.rs:1 fn foo<F, X>(cb: F) -> X
src/main.rs:2     where F : for<'a> FnOnce(&'a u64) -> X {
src/main.rs:3     foo(move |expr1| {
src/main.rs:4         cb(expr1)
src/main.rs:5     })
src/main.rs:6 }
