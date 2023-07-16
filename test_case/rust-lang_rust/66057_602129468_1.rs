
error[E0271]: type mismatch resolving `<std::option::Option<char> as std::iter::IntoIterator>::Item == &'static str`
 --> src/main.rs:6:12
  |
6 |     make().extend(Some('c'));
  |            ^^^^^^ expected `char`, found `&str`

error: aborting due to previous error
