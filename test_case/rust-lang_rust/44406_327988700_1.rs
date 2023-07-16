
Standard Error

   Compiling playground v0.0.1 (file:///playground)
error: expected identifier, found keyword `true`
 --> src/main.rs:8:10
  |
8 |     foo!(true);
  |          ^^^^

error: `<` is interpreted as a start of generic arguments for `true`, not a comparison
 --> src/main.rs:3:23
  |
3 |           bar(baz: $rest)
  |                         -
  |                         |
  |  _______________________not interpreted as comparison
  | |
4 | |     }
5 | | }
6 | |
7 | | fn main() {
8 | |     foo!(true);
  | |     -----------
  | |_____|_______|
  |       |       interpreted as generic arguments
  |       in this macro invocation
  |
help: if you want to compare the casted value then write:
  |
3 |         bar((baz: $rest)
4 |     }
5 | }
6 | 
7 | fn main() {
8 |     foo!(true));
  |

error: aborting due to 2 previous errors

error: Could not compile `playground`.

To learn more, run the command again with --verbose.
