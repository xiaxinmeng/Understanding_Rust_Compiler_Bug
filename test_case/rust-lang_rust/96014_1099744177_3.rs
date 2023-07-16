
error: `macro_rules!` macros do not support single-matcher shorthand syntax
 --> src/lib.rs:1:35
  |
1 | macro_rules! count_tts($($t:tt)*) {
  |                       ^^^^^^^^^^^
  |
help: use the standard syntax instead
  |
1 ~ macro_rules! count_tts {
2 +     ($($t:tt)*) => {
3 ~         // TODO
4 ~         0
5 +     };
6 | }
  |
