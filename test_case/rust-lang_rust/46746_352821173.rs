
error: internal compiler error: /home/pnkfelix/Dev/Mozilla/rust-mirborrowck/src/librustc_mir/borrow_check/mod.rs:342: mutable borrows must be assigned to locals
  --> /home/pnkfelix/Dev/Mozilla/rust-mirborrowck/src/test/compile-fail/borrowck/borrowck-issue-14498.rs:27:25
   |
27 |     let y: Box<_> = box &mut x;
   |                         ^^^^^^

note: the compiler unexpectedly panicked. this is a bug.
