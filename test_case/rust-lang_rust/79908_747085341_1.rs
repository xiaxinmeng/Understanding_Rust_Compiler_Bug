
   Compiling playground v0.0.1 (/playground)
error: expected expression, found keyword `const`
  --> src/main.rs:17:15
   |
8  |     (exp $e:expr) => {
   |          ------- while parsing argument for this `expr` macro fragment
...
17 |     stmt!(exp const 1);
   |               ^^^^^ expected expression

warning: unused macro definition
 --> src/main.rs:1:1
  |
1 | / macro_rules! exp {
2 | |     (const $n:expr) => {
3 | |         $n
4 | |     };
5 | | }
  | |_^
  |
  = note: `#[warn(unused_macros)]` on by default

error: aborting due to previous error; 1 warning emitted

error: could not compile `playground`

To learn more, run the command again with --verbose.
