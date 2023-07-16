
error[E0597]: `split` does not live long enough
  --> src/main.rs:18:25
   |
18 |         let mut split = split.split(":");
   |                         ^^^^^ borrowed value does not live long enough
...
22 |         map.insert(name, value);
   |         --- borrow later used here
23 |     }
   |     - `split` dropped here while still borrowed
