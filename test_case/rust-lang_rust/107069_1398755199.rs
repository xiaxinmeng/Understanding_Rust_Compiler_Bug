console
   Compiling issue-11598 v0.1.0 (/home/user/projects/issue-11598)
     Running `rustc --crate-name build_script_build --edition=2021 src/build.rs ...`
error[E0432]: unresolved import `crate::Lib`
 --> src/foo.rs:2:5
  |
2 | use crate::Lib;
  |     ^^^^^^^^^^ no `Lib` in the root

For more information about this error, try `rustc --explain E0432`.
error: could not compile `issue-11598` due to previous error

Caused by:
  process didn't exit successfully: `rustc --crate-name build_script_build --edition=2021 src/build.rs ...` (exit status: 1)
