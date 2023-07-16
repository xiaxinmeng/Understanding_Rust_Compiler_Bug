
error[E0502]: cannot borrow `parents` as mutable because it is also borrowed as immutable
  --> src/main.rs:16:9
   |
16 |         parents.push(Parent);
   |         ^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
17 |         let p = parents.last().unwrap();
   |                 ------- immutable borrow occurs here
18 |         
19 |         children.push(Child(None));
   |         -------- immutable borrow used here, in later iteration of loop
