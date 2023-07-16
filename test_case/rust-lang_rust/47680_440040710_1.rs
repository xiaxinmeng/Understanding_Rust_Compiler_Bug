
error[E0499]: cannot borrow `list.next` as mutable more than once at a time
  --> src/lib.rs:10:25
   |
5  | fn example(mut list: &mut Node) {
   |                      - let's call the lifetime of this reference `'1`
6  |     if let Some(node) = &mut list.next {
   |                 ----    -------------- first mutable borrow occurs here
   |                 |
   |                 assignment requires that `list.next` is borrowed for `'1`
...
10 |     if let Some(node) = &mut list.next {
   |                         ^^^^^^^^^^^^^^ second mutable borrow occurs here
