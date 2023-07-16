
error[E0499]: cannot borrow `txt` as mutable more than once at a time
  --> src/main.rs:21:12
   |
17 |       ptr: &mut txt,
   |            -------- first mutable borrow occurs here
...
21 |   use_text(&mut txt);
   |            ^^^^^^^^
   |            |
   |            second mutable borrow occurs here
   |            first borrow is still live here
