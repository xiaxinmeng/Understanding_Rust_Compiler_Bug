
error[E0716]: temporary value dropped while borrowed
  --> file.rs:10:19
   |
LL |     if check_val(&n.to_string()) || check_val("Hello") {}
   |                   ^^^^^^^^^^^^^-    --------- borrow later used here
   |                   |            |
   |                   |            temporary value is freed at the end of this statement
   |                   creates a temporary which is freed while still in use
   |
   = note: consider using a `let` binding to create a longer lived value
