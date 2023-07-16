rust
error[E0506]: cannot assign to `list` because it is borrowed
  --> /Users/david/Documents/Code/rust-nll/src/test/run-pass/nll-iterating-and-updating.rs:25:13
   |
23 |         result.push(&mut list.value);
   |                     --------------- borrow of `list` occurs here
24 |         if let Some(n) = list.next.as_mut() {
25 |             list = n;
   |             ^^^^^^^^ assignment to borrowed `list` occurs here

error: aborting due to previous error
