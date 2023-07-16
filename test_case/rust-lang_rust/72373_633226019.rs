
error: expected one of `,`, `@`, `]`, or `|`, found `..`
 --> src/main.rs:5:19
  |
5 |         [h, ref ts..] => foo(c, n - h) + foo(ts, n),
  |                   ^^
  |                   |
  |                   expected one of `,`, `@`, `]`, or `|`
  |                   help: maybe missing `@` in binding pattern?

error: aborting due to previous error
