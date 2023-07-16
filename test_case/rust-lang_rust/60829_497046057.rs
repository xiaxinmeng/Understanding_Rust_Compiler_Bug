
error: incorrect use of `await`
  --> src/main.rs:12:11
   |
12 |     match await { await => () } //~ ERROR expected `!`, found `{`
   |           ^^^^^^^^^^^^^^^^^^^^^ help: `await` is a postfix operation: `{ await => () }.await`

error: expected one of `.`, `?`, `{`, or an operator, found `}`
  --> src/main.rs:13:1
   |
12 |     match await { await => () } //~ ERROR expected `!`, found `{`
   |     -----                      - expected one of `.`, `?`, `{`, or an operator here
   |     |
   |     while parsing this match expression
13 | }
   | ^ unexpected token
