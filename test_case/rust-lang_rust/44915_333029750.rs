
   Compiling playground v0.0.1 (file:///playground)
error: expected one of `:`, `@`, or `|`, found `)`
 --> src/main.rs:7:15
  |
7 |         foo(|y) {
  |               ^ expected one of `:`, `@`, or `|` here

error: expected one of `.`, `;`, `?`, `}`, or an operator, found `{`
 --> src/main.rs:7:17
  |
7 |         foo(|y) {
  |                -^ unexpected token
  |                |
  |                expected one of `.`, `;`, `?`, `}`, or an operator here

error[E0061]: this function takes 1 parameter but 0 parameters were supplied
 --> src/main.rs:7:9
  |
1 | / fn foo<F>(f: F) where F: FnOnce(usize) {
2 | |     f(0)
3 | | }
  | |_- defined here
...
7 |           foo(|y) {
  |           ^^^^^^^ expected 1 parameter

error: aborting due to 3 previous errors

error: Could not compile `playground`.

To learn more, run the command again with --verbose.
