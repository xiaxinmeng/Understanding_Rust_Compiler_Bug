
error: expected one of `.`, `;`, `?`, or an operator, found `""`
 --> test.rs:3:19
  |
3 |         let x = 5 "";
  |                  -^^ unexpected token
  |                  |
  |                  expected one of `.`, `;`, `?`, or an operator here

error: macro expansion ignores token `}` and any following
 --> test.rs:4:5
  |
4 |     }}
  |     ^
  |
note: caused by the macro expansion here; the usage of `failed!` is likely invalid in statement context
 --> test.rs:8:5
  |
8 |     failed!();
  |     ^^^^^^^^^^

error: aborting due to 2 previous errors
