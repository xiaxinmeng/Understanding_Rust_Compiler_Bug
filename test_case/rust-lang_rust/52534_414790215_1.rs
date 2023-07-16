
error[E0597]: `x` does not live long enough
  --> src/main.rs:9:14
   |
8  |     let x = 22;
   |        - value defined here
9  |     foo(|a| &x)
   |         ---  ^ borrowed value does not live long enough
   |         |
   |         value captured here
10 | }
   | - `x` dropped here while still borrowed
   |
   = note: borrowed value must be valid for the `'static` lifetime...
   = note: ...because the closure cannot find any lifetime in its arguments that would be applicable, so it requires `'static`
