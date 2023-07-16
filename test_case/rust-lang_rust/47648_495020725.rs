
error[E0525]: expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce`
  --> src/main.rs:11:19
   |
11 |     let closure = move || foo;
   |                   ^^^^^^^^---
   |                   |       |
   |                   |       closure is `FnOnce` because it moves the variable `foo` out of its environment
   |                   this closure implements `FnOnce`, not `Fn`
12 | 
13 |     call(&closure);
   |     ---- the requirement to implement `Fn` derives from here
