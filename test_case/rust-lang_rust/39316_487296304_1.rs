
error[E0502]: cannot borrow `x` as immutable because it is also borrowed as mutable
  --> src/main.rs:30:5
   |
29 |     let v: &u8 = x.func1(); // Here, we borrow x mutably...
   |                  - mutable borrow occurs here
30 |     x.func2(v); // ... and here immutably.
   |     ^       - mutable borrow later used here
   |     |
   |     immutable borrow occurs here
