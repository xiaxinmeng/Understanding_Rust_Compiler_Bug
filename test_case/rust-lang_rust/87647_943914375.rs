
error: expected `;`, found `}`
 --> src/main.rs:4:17
  |
4 |     pub fn bar()
  |                 ^ help: add `;` here
5 | }
  | - unexpected token

error: associated function in `impl` without body
 --> src/main.rs:4:5
  |
4 |     pub fn bar()
  |     ^^^^^^^^^^^-
  |                |
  |                help: provide a definition for the function: `{ <body> }`

error: could not compile `playground` due to 2 previous errors
