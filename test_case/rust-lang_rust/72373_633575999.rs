
error: expected one of `,`, `@`, `]`, or `|`, found `..`
 --> src/main.rs:5:20
  |
5 |         [h, ref ts..] => foo(c, n - h) + foo(ts, n),
  |                    ^^ expected one of `,`, `@`, `]`, or `|`
  |
help: if you meant to bind the contents of the rest of the array pattern into `ts`, use `@`
  |
5 |         [h, ref ts @ ..] => foo(c, n - h) + foo(ts, n),
  |                    ^

error: aborting due to previous error
