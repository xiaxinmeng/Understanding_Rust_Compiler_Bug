
error: expected expression, found `.`
 --> src/main.rs:6:12
  |
6 |     test!{}.len()
  |            ^ expected expression

error[E0308]: mismatched types
 --> src/main.rs:2:13
  |
2 |     () => ( "hello" )
  |             ^^^^^^^ expected usize, found reference
...
5 | fn string_len() -> usize {
  |                    ----- expected `usize` because of return type
6 |     test!{}.len()
  |     ------- in this macro invocation
  |
  = note: expected type `usize`
             found type `&'static str`
