
warning: unreachable arm
 --> /checkout/src/test/run-fail/match-disc-bot.rs:7:17
  |
6 |     match f() {
  |           --- any code following this expression is unreachable
7 |         true => 1,
  |                 ^ unreachable arm
  |
  = note: `#[warn(unreachable_code)]` on by default

error[E0004]: non-exhaustive patterns: `_` not covered
 --> /checkout/src/test/run-fail/match-disc-bot.rs:6:11
  |
6 |     match f() {
  |           ^^^ pattern `_` not covered
  |
  = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
  = note: the matched value is of type `!`

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0004`.
