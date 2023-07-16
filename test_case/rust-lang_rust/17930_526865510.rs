

error: expected expression, found `as`
 --> src/main.rs:3:18
  |
3 |     unsafe { 0 } as i32;  // expected identifier, found keyword `as`
  |     ------------ ^^ expected expression
  |     |
  |     help: parentheses are required to parse this as an expression: `(unsafe { 0 })`

error: aborting due to previous error
