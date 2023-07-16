
test.rs:2:6: 2:13 error: `$a:expr` is followed by `|`, which is not allowed for `expr` fragments
test.rs:2     ($a:expr | $b:expr) => (
               ^~~~~~~
test.rs:8:15: 8:16 error: unexpected end of macro invocation
test.rs:8     foo!(2u | 3);
