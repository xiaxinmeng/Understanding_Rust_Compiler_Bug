
error: `z` does not live long enough
  --> src/main.rs:14:27
   |
12 |                             Some(z) => &z,
   |                                         - borrow occurs here
13 |                             None => panic!(),
14 |                           };
   |                           ^ `z` dropped here while still borrowed
...
23 | }
   | - borrowed value needs to live until here
