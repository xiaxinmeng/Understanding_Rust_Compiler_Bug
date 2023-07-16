
error[E0597]: `child` does not live long enough
  --> src/main.rs:55:43
   |
49 |             Ok(child) => members.push(child.raw),
   |                                       ^^^^^^^^^ borrowed value does not live long enough
50 |             Err(_) => ()
57 |         }
   |         - the destructor for `child` executes here
58 |     }
59 |     members.len();
   |     ------- borrow of `child.raw` continues to exist here
   |
note: `child` has a destructor implementation that might access `child.raw`
   |
15 | / impl<'a> Drop for Child<'a> {
16 | |     fn drop(&mut self) { }
17 | | }
   | |_^
   = note: <some further explanation or link to explain `Drop`>
