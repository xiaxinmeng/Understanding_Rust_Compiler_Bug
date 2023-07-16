
error: internal compiler error: src/librustc_typeck/check/mod.rs:4498: parenthesized parameters cannot appear in ExprPath
 --> test.rs:2:19
  |
2 |     ($p:path) => ($p);
  |                   ^^
...
5 |     println!("{}", pathexpr!(std::str::from_utf8())(b"Hello").unwrap());
  |                    -------------------------------- in this macro invocation
