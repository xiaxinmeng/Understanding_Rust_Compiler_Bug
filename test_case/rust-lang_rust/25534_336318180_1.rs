
error[E0505]: cannot move out of `a` because it is borrowed
 --> src/main.rs:4:18
  |
3 |     for i in &a {
  |               - borrow of `a` occurs here
4 |         for j in a {
  |                  ^ move out of `a` occurs here
  |                  |
  |                  help: consider borrowing `a` instead: `&a`

error[E0382]: use of moved value: `a`
 --> src/main.rs:4:18
  |
4 |         for j in a {
  |                  ^ value moved here in previous iteration of loop
  |                  |
  |                  help: consider borrowing `a` instead: `&a`
  = note: move occurs because `a` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait

error: aborting due to 2 previous errors
