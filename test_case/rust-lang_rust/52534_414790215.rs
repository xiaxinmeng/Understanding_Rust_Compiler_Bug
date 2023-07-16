
error[E0597]: `x` does not live long enough
  --> src/main.rs:9:14
   |
9  |     foo(|a| &x)
   |         ---  ^ borrowed value does not live long enough
   |         |
   |         value captured here
10 | }
   | - `x` dropped here while still borrowed
   |
   = note: borrowed value must be valid for the static lifetime...
