
error: unsatisfied lifetime constraints
 --> src/main.rs:5:8
  |
5 |     || doit(data);
  |     -- ^^^^^^^^^^ argument requires that `'1` must outlive `'static`
  |     |
  |     lifetime `'1` represents this closure's body
  |
  = note: closure implements `FnMut`, so references to captured variables can't escape the closure

error[E0597]: `data` does not live long enough
 --> src/main.rs:5:13
  |
5 |     || doit(data);
  |     -- -----^^^^-
  |     |  |    |
  |     |  |    borrowed value does not live long enough
  |     |  argument requires that `data` is borrowed for `'static`
  |     value captured here
6 | }
  |  - `data` dropped here while still borrowed
