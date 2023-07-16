
error[E0382]: use of moved value: `a`
 --> src/main.rs:4:18
  |
3 |     for i in a {
  |              - value moved here
  |              |
  |              help: consider borrowing `a` instead: `&a`
4 |         for j in a {
  |                  ^ value used here after move
  |                  |
  |                  help: consider borrowing `a` instead: `&a`
  = note: move occurs because `a` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait
