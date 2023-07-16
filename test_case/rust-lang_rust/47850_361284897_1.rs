
error[E0507]: cannot move out of captured outer variable in an `FnMut` closure
  --> src/main.rs:20:19
   |
17 |     let a = &mut foo.a;
   |         - captured outer variable
...
20 |         *if true {a} else {b} = 42;
   |                   ^ cannot move out of captured outer variable in an `FnMut` closure
note: closure is `FnMut` because of the requirements of `f()`
  --> src/main.rs:19:19
   |
19 |         foo.bar.f(||
   |         ^^^^^^^^^
   |            
