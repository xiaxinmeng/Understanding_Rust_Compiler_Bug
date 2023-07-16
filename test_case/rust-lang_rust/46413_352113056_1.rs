
   Compiling playground v0.0.1 (file:///playground)
error[E0597]: `stmt` does not live long enough
  --> src/main.rs:26:1
   |
19 |     let mut rows = Rows(&stmt);
   |                          ---- borrow occurs here
...
26 | }
   | ^ `stmt` dropped here while still borrowed
   |
   = note: values in a scope are dropped in the opposite order they are created
