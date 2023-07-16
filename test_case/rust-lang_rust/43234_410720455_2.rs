
error[E0597]: `y` does not live long enough
  --> src/main.rs:8:15
   |
8  |         x.0 = &y;
   |               ^^ borrowed value does not live long enough
9  |         println!("{:?}", x);
10 |     }
   |     - `y` dropped here while still borrowed
...
14 |         x.0 = &y;
   |         -------- borrow later used here
