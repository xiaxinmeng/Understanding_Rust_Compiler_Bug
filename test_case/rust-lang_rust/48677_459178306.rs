
error[E0502]: cannot borrow `*list` as mutable because it is also borrowed as immutable
 --> src/lib.rs:6:5
  |
3 |     for v in list.iter() {
  |              ---- immutable borrow occurs here
...
6 |     list.push(T::default());
  |     ^^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
7 |     drop(cloned_items);
  |          ------------ immutable borrow later used here

error: aborting due to previous error
