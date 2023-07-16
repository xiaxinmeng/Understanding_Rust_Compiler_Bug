
gh-vincenzopalazzo@dev-desktop-eu-2:~/rust$ ./build/x86_64-unknown-linux-gnu/stage1/bin/rustc test.rs 
error[E0277]: the trait bound `MySnafu: Try` is not satisfied
 --> test.rs:8:10
  |
8 |     impl FromResidual for MySnafu {
  |          ^^^^^^^^^^^^ the trait `Try` is not implemented for `MySnafu`

error[E0277]: the trait bound `MySnafu: Try` is not satisfied
  --> test.rs:9:9
   |
9  | /         fn from_residual(s: Self) -> Self {
10 | |             todo!()
11 | |         }
   | |_________^ the trait `Try` is not implemented for `MySnafu`

error: aborting due to 2 previous errors

