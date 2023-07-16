
error[E0502]: cannot borrow `tmp` as mutable because it is also borrowed as immutable
  --> src/main.rs:28:13
   |
21 |     match tmp.get_iter() { // `xx` holds the reference to `tmp`
   |           --- immutable borrow occurs here
...
28 |             tmp.push(1);
   |             ^^^^^^^^^^^ mutable borrow occurs here
...
31 |     };
   |     - borrow later used here, when `_yy` is dropped
