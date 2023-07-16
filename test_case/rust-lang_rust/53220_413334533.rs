
error[E0597]: `child` does not live long enough
  --> src/main.rs:55:43
   |
55 |             Ok(mut child) => members.push(child.raw()),
   |                                           ^^^^^ borrowed value does not live long enough
56 |             Err(_) => ()
57 |         }
   |         - `child` dropped here while still borrowed
58 |     }
59 |     members.len();
   |     ------- borrow used here in later iteration of loop
