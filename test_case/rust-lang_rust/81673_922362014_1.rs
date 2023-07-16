
error[E0506]: cannot assign to `foo` because it is borrowed
 --> src/main.rs:7:9
  |
3 |     let r = &foo;
  |             ---- borrow of `foo` occurs here
...
6 |         println!("{}", r);
  |                        - borrow later used here
7 |         foo += 1;
  |         ^^^^^^^^ assignment to borrowed `foo` occurs here
