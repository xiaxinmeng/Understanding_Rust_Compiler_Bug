
error[E0597]: borrowed value does not live long enough
 --> src/main.rs:9:21
  |
9 |     let a: Vec<_> = files.write().unwrap().drain(..1).collect();
  |                     ^^^^^^^^^^^^^^^^^^^^^^                     - temporary value dropped here while still borrowed
  |                     |
  |                     temporary value does not live long enough
  |
  = note: values in a scope are dropped in the opposite order they are created
  = note: consider using a `let` binding to increase its lifetime
