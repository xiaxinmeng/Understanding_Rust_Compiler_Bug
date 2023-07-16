rust
warning: cannot borrow `counts` as mutable because it is also borrowed as immutable
  --> src/day_2/part_1.rs:23:29
   |
18 |             .for_each(|c| match counts.get(&c) {
   |                                 ------ immutable borrow occurs here
...
23 |                     let _ = counts.insert(c, *t + 1);
   |                             ^^^^^^           -- immutable borrow later used here
   |                             |
   |                             mutable borrow occurs here
   |
   = note: `#[warn(mutable_borrow_reservation_conflict)]` on by default
   = warning: this borrowing pattern was not meant to be accepted, and may become a hard error in the future
