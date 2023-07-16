
error[E0525]: expected a closure that implements the `FnMut` trait, but this closure only implements `FnOnce`
  --> src/main.rs:19:19
   |
19 |       let closure = || { // doesn't work
   |  ___________________^
20 | |         *if true {a} else {b} = 42;
21 | |     };
   | |_____^
22 |       foo.bar.f(closure);
   |               - the requirement to implement `FnMut` derives from here
   |
note: closure is `FnOnce` because it moves the variable `a` out of its environment
  --> src/main.rs:20:19
   |
20 |         *if true {a} else {b} = 42;
   |                
