
$ cat const.rs 
const i: i32 = {
    1 + 2;
    1
};
$ rustdoc +stable const.rs 
warning: constant `i` should have an upper case name
 --> const.rs:1:7
  |
1 | const i: i32 = {
  |       ^ help: convert the identifier to upper case (notice the capitalization): `I`
  |
  = note: `#[warn(non_upper_case_globals)]` on by default

warning: unused arithmetic operation that must be used
 --> const.rs:2:5
  |
2 |     1 + 2;
  |     ^^^^^
  |
  = note: `#[warn(unused_must_use)]` on by default
$ rustdoc +stage1 const.rs
// no output
$
