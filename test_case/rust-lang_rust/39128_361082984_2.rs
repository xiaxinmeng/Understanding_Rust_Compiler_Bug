
error[E0507]: cannot move out of borrowed content
  --> src/main.rs:18:49
   |
18 |                 S(ref lhs) => true && (*lhs) == (*rhs)
   |                                                 ^^^^^^ cannot move out of borrowed content
