
⌁ rustc 4536.rs
4536.rs:4:12: 4:21 error: failed to resolve import: super::S
4536.rs:4         use super::S;
                      ^~~~~~~~~
4536.rs:5:12: 5:19 error: failed to resolve import: foo::S
4536.rs:5         use foo::S;
                      ^~~~~~~
error: failed to resolve imports
error: aborting due to 3 previous errors
