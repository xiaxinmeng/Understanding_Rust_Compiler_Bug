
error: `x` does not live long enough
  --> src/test/compile-fail/regions-escape-loop-via-variable.rs:21:14
   |
21 |         p = &x;
   |              - borrow occurs here
22 |     }
   |     ^ `x` dropped here while still borrowed
23 | }
   | - borrowed value needs to live until here
