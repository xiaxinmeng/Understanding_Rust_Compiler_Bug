
error[E0597]: `c` does not live long enough
  --> src/main.rs:40:25
   |
40 |     let d = init_d(&mut c);
   |                         ^ borrowed value does not live long enough
...
43 | }
   | - `c` dropped here while still borrowed
   |
   = note: values in a scope are dropped in the opposite order they are created

warning: variable does not need to be mutable
  --> src/main.rs:38:9
   |
38 |     let mut c = init_c(&mut b);
   |         ---^^
   |         |
   |         help: remove this `mut`
   |
   = note: #[warn(unused_mut)] on by default
