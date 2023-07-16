
error[E0597]: `a` does not live long enough
 --> src/main.rs:7:30
  |
4 |       let x = {
  |           - borrow later stored here
5 |           let a = 5;
6 |           async {
  |  _______________-
7 | |             println!("{:?}", a);
  | |                              ^ borrowed value does not live long enough
8 | |         }
  | |_________- value captured here by generator
9 |       };
  |       - `a` dropped here while still borrowed
