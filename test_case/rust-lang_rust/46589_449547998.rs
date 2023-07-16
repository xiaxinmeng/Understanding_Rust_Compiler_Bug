

error[E0499]: cannot borrow `**other` as mutable more than once at a time
  --> src/main.rs:19:21
   |
17 |         *other = match (*other).get_self() {
   |                        -------- first mutable borrow occurs here
18 |             Some(s) => s,
19 |             None => (*other).new_self()
   |                     ^^^^^^^^
   |                     |
   |                     second mutable borrow occurs here
   |                     first borrow later used here

error: aborting due to previous error
