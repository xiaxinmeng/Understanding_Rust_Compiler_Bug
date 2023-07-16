
error[E0597]: `arg` does not live long enough
  --> src/main.rs:6:24
   |
4  |     let _arg = match Some(()) {
   |         ---- borrow later stored here
5  |         Some(arg) => {
6  |             match Some(&arg) {
   |                        ^^^^ borrowed value does not live long enough
...
11 |         None => return,
   |            - `arg` dropped here while still borrowed
