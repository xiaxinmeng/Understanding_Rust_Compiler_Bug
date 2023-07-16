
error[E0499]: cannot borrow `tx` as mutable more than once at a time
  --> src/lib.rs:42:5
   |
41 |     tx.scan().for_each(|x| println!("{:?}", x));
   |     -- first mutable borrow occurs here
42 |     tx.scan().for_each(|x| println!("{:?}", x));
   |     ^^
   |     |
   |     second mutable borrow occurs here
   |     first borrow later used here

error: aborting due to previous error
