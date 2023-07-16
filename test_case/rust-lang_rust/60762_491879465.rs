
error: expected one of `.`, `;`, `?`, or an operator, found `#`
 --> src/main.rs:2:25
  |
2 |     let foo = r##"bar"###;
  |                --       ^
  |                |        |
  |                |        the raw string needs 2 trailing `#`, but found 3
  |                |        help: remove this `#` (this should be a hidden suggestion for rustfix' sake)
  |                the raw string has 2 leading `#`
