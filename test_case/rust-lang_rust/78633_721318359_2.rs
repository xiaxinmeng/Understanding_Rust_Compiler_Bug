
error[E0597]: `arena` does not live long enough
  --> src/main.rs:10:20
   |
7  |     let mut track = Vec::new();
   |         --------- lifetime `'1` appears in the type of `track`
...
10 |         track.push(&arena)
   |         -----------^^^^^^-
   |         |          |
   |         |          borrowed value does not live long enough
   |         argument requires that `arena` is borrowed for `'1`
11 |     }
   |     - `arena` dropped here while still borrowed
