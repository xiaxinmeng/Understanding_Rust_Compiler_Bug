
error[E0597]: `x` does not live long enough
  --> src/main.rs:9:14
   |
8  |     let x = 22;
   |        - value defined here which is valid until the end of the scope
9  |     foo(|a| &x)
   |         ---  ^ borrowed value does not live long enough
   |         |
   |         value captured here
10 | }
   | - `x` dropped here while still borrowed
   |
   = note: borrowed value must be valid for the `'static` lifetime...
note: ...because `x` is captured by the closure...
  |
9 |     foo(|a| &x)
  |             ^^
note: ...but the closure outlives the current scope when passed into `foo`
  |
9 |     foo(|a| &x)
  |     ^^^
