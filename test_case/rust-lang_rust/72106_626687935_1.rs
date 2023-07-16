
error: `impl` item signature doesn't match `trait` item signature
 --> src/lib.rs:6:5
  |
2 |     fn foo(value: &str) -> Self;
  |     ---------------------------- expected fn(&str) -> &str
...
6 |     fn foo(value: &str) -> &str {
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ found fn(&str) -> &str
  |
  = note: expected `fn(&str) -> &str`
             found `fn(&str) -> &str`
