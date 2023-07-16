
error[E0502]: cannot borrow `parents` as mutable because it is also borrowed as immutable
  --> src/main.rs:9:9
   |
9  |         parents.push(Parent);
   |         ^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
10 |         let p = parents.last().unwrap();
   |                 -------------- immutable borrow occurs here
11 |         
12 |         children.push(Child(None));
   |         -------------------------- immutable borrow later used here
