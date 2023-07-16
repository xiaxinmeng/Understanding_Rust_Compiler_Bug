
[01:27:33] error[E0107]: wrong number of type arguments: expected 2, found 1
[01:27:33]    --> src/tools/miri/src/stacked_borrows.rs:362:6
[01:27:33]     |
[01:27:33] 362 | impl AllocationExtra<Borrow> for Stacks {
[01:27:33]     |      ^^^^^^^^^^^^^^^^^^^^^^^ expected 2 type arguments
[01:27:33] 
[01:27:33] error: aborting due to previous error
