
[17:03:53]/tmp> rustc foo.rs
foo.rs:4:12: 4:20 error: unresolved import: found `S` in `foo` but it is private
foo.rs:4         use super::S;
                     ^~~~~~~~
foo.rs:4:12: 4:20 error: failed to resolve import `super::S`
foo.rs:4         use super::S;
                     ^~~~~~~~
