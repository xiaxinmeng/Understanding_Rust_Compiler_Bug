
error[E0597]: `s` does not live long enough
  --> src/main.rs:17:16
   |
17 |         T{ s: &s }.s.n
   |                ^ borrowed value does not live long enough
18 |     
19 | }
   | - `s` dropped here while still borrowed
   |
   = note: values in a scope are dropped in the opposite order they are created
