rs
error[E0597]: `x` does not live long enough
  --> src/main.rs:6:17
   |
6  |         closure(&x);
   |                 ^^ borrowed value does not live long enough
7  |     }
   |     - `x` dropped here while still borrowed
...
10 |         closure(&x);
   |         ------- borrow later used here
