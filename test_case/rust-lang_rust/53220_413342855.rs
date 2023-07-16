
error[E0597]: `child` does not live long enough
  --> src/main.rs:55:43
   |
49 |             Ok(child) => members.push(child.raw),
   |                                       ^^^^^^^^^ borrowed value does not live long enough
50 |             Err(_) => ()
57 |         }
   |         - the destructor for `child` executes here, and it may access `child.raw`
58 |     }
59 |     members.len();
   |     ------- borrow used here in later iteration of loop
   |
  = note: the drop impl appears here
   |    impl Drop for ... { .. }
