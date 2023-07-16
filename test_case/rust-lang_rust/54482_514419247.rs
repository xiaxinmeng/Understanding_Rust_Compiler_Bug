
error: expected expression, found `+`
 --> src/lib.rs:6:9
  |
6 |     {2} + {2}
  |     --- ^ expected expression
  |     |
  |     help: parentheses are required to parse this as an expression: `({2})`

error: expected expression, found `+`
  --> src/lib.rs:14:9
   |
14 |     {2} + 2
   |     --- ^ expected expression
   |     |
   |     help: parentheses are required to parse this as an expression: `({2})`

error[E0308]: mismatched types
 --> src/lib.rs:6:6
  |
6 |     {2} + {2}
  |      ^ expected (), found integer
  |
  = note: expected type `()`
             found type `{integer}`

error[E0308]: mismatched types
  --> src/lib.rs:14:6
   |
14 |     {2} + 2
   |      ^ expected (), found integer
   |
   = note: expected type `()`
              found type `{integer}`

error: aborting due to 4 previous errors
