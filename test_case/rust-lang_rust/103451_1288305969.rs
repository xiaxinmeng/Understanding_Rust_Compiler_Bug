console
cat@LAPTOP-V6U0QKD4:~/code/rust$ cat hang.rs
struct R { }

struct S {
    x: [u8; R
cat@LAPTOP-V6U0QKD4:~/code/rust$ rustc -vV
rustc 1.66.0-nightly (7fcf850d7 2022-10-23)
binary: rustc
commit-hash: 7fcf850d7942804990a1d2e3fe036622a0fe4c74
commit-date: 2022-10-23
host: x86_64-unknown-linux-gnu
release: 1.66.0-nightly
LLVM version: 15.0.2
cat@LAPTOP-V6U0QKD4:~/code/rust$ rustc hang.rs
error: this file contains an unclosed delimiter
 --> hang.rs:4:15
  |
3 | struct S {
  |          - unclosed delimiter
4 |     x: [u8; R
  |        -      ^
  |        |
  |        unclosed delimiter

error[E0423]: expected value, found struct `R`
 --> hang.rs:4:13
  |
1 | struct R { }
  | ------------ `R` defined here
...
4 |     x: [u8; R
  |             ^ help: use struct literal syntax instead: `R {}`

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0423`.
