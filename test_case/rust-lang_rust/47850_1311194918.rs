
error[E0507]: cannot move out of `a`, a captured variable in an `FnMut` closure
  --> src/main.rs:27:19
   |
24 |     let a = &mut foo.a;
   |         - captured outer variable
25 |     let b = &mut foo.b;
26 |     foo.bar.f(|| { // doesn't work
   |               -- captured by this `FnMut` closure
27 |         *if true {a} else {b} = 42;
   |                   ^ move occurs because `a` has type `&mut i32`, which does not implement the `Copy` trait

error[E0507]: cannot move out of `b`, a captured variable in an `FnMut` closure
  --> src/main.rs:27:28
   |
25 |     let b = &mut foo.b;
   |         - captured outer variable
26 |     foo.bar.f(|| { // doesn't work
   |               -- captured by this `FnMut` closure
27 |         *if true {a} else {b} = 42;
   |                            ^ move occurs because `b` has type `&mut i32`, which does not implement the `Copy` trait
