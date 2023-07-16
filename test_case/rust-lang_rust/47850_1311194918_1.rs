
error[E0525]: expected a closure that implements the `FnMut` trait, but this closure only implements `FnOnce`
  --> src/main.rs:19:19
   |
19 |     let closure = || { // doesn't work
   |                   ^^ this closure implements `FnOnce`, not `FnMut`
20 |         *if true {a} else {b} = 42;
   |                   - closure is `FnOnce` because it moves the variable `a` out of its environment
21 |     };
22 |     foo.bar.f(closure);
   |             - ------- the requirement to implement `FnMut` derives from here
   |             |
   |             required by a bound introduced by this call
   |
note: required by a bound in `Bar::f`
  --> src/main.rs:10:13
   |
10 |     fn f<F: FnMut()>(&self, mut f: F) {
   |             ^^^^^^^ required by this bound in `Bar::f`
