
error[E0502]: cannot borrow `**self.current` as immutable because it is also borrowed as mutable
  --> src/main.rs:43:51
   |
43 |                 mem::replace(&mut self.current, &*(**self.current).get_pointer(0));
   |                 ----------------------------------^^^^^^^^^^^^^^^^----------------
   |                 |            |                    |
   |                 |            |                    immutable borrow occurs here
   |                 |            mutable borrow occurs here
   |                 borrow later used here
