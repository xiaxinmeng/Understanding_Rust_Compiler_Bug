
error: lifetime may not live long enough
 --> src/lib.rs:3:8
  |
3 |     || doit(data);
  |     -- ^^^^^^^^^^ argument requires that `'1` must outlive `'static`
  |     |
  |     lifetime `'1` represents this closure's body
  |
  = note: closure implements `FnMut`, so references to captured variables can't escape the closure

error[E0597]: `data` does not live long enough
 --> src/lib.rs:3:13
  |
3 |     || doit(data);
  |     -- -----^^^^-
  |     |  |    |
  |     |  |    borrowed value does not live long enough
  |     |  argument requires that `data` is borrowed for `'static`
  |     value captured here
4 | }
  |  - `data` dropped here while still borrowed
