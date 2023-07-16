
error[E0597]: `dests` does not live long enough
  --> src/main.rs:11:34
   |
11 |     let pairs = paths.iter().zip(dests.iter());
   |                                  ^^^^^ borrowed value does not live long enough
...
14 | }
   | - `dests` dropped here while still borrowed
   |
   = note: values in a scope are dropped in the opposite order they are created

error: aborting due to previous error
