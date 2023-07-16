
error[E0597]: `pb` does not live long enough
  --> src/main.rs:6:23
   |
6  |     let p = Path::new(&pb);
   |             ----------^^^-
   |             |         |
   |             |         borrowed value does not live long enough
   |             argument requires that `pb` is borrowed for `'static`
...
10 | }
   | - `pb` dropped here while still borrowed
