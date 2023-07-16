
error[E0502]: cannot borrow `*self` as mutable because it is also borrowed as immutable
  --> src/main.rs:19:7
   |
18 |     for datum in &self.data.vector {
   |                  -----------------
   |                  ||
   |                  |immutable borrow occurs here
   |                  immutable borrow later used here
19 |       self.process_datum(datum);
   |       ^^^^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
