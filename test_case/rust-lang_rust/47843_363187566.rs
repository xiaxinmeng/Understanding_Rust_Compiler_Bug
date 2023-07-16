
error[E0503]: cannot use `x` because it was mutably borrowed
 --> src/main.rs:4:9
  |
3 |     let y = &mut x;
  |                  - borrow of `x` occurs here
4 |     let z = x;
  |         ^ use of borrowed `x`
