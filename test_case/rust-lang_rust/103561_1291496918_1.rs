
   Compiling playground v0.0.1 (/playground)
error: expected identifier, found keyword `const`
 --> src/main.rs:2:9
  |
2 |     for const i in 0..10 {
  |         ^^^^^ expected identifier, found keyword

error: missing `in` in `for` loop
 --> src/main.rs:2:14
  |
2 |     for const i in 0..10 {
  |              ^ help: try adding `in` here

error: expected `{`, found keyword `in`
 --> src/main.rs:2:17
  |
2 |     for const i in 0..10 {
  |                 ^^ expected `{`

error: could not compile `playground` due to 3 previous errors
