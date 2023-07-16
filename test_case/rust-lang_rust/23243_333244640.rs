
error: expected expression, found keyword `ref`
 --> src/main.rs:3:20
  |
3 |     (pattern) => { ref _x }
  |                    ^^^
...
6 |     foo!(pattern);
  |     -------------- in this macro invocation

error: expected pattern, found keyword `fn`
 --> src/main.rs:2:17
  |
2 |     (item) => { fn not_a_pattern() {} };
  |                 ^^
...
7 |     let foo!(item) = ();
  |         ---------- in this macro invocation
