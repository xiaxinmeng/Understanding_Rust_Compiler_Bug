
gh-aDotInTheVoid@dev-desktop-eu-1:~/tmp/ice-110447$ cargo +beta b
   Compiling ice-110447 v0.1.0 (/home/gh-aDotInTheVoid/tmp/ice-110447)
error[E0631]: type mismatch in function arguments
 --> src/main.rs:7:47
  |
1 | fn score(_: (usize, &str)) -> usize {
  | ----------------------------------- found signature defined here
...
7 |     let result = names.iter().enumerate().map(score);
  |                                           --- ^^^^^ expected due to this
  |                                           |
  |                                           required by a bound introduced by this call
  |
  = note: expected function signature `fn((_, &&str)) -> _`
             found function signature `for<'a> fn((_, &'a str)) -> _`
note: required by a bound in `map`
 --> /rustc/f18236dcd3d8191c91aca0c4ef43e1e27b6bc0dc/library/core/src/iter/traits/iterator.rs:796:5

For more information about this error, try `rustc --explain E0631`.
error: could not compile `ice-110447` due to previous error
gh-aDotInTheVoid@dev-desktop-eu-1:~/tmp/ice-110447$ cargo +nightly b
   Compiling ice-110447 v0.1.0 (/home/gh-aDotInTheVoid/tmp/ice-110447)
error[E0631]: type mismatch in function arguments
 --> src/main.rs:7:47
  |
1 | fn score(_: (usize, &str)) -> usize {
  | ----------------------------------- found signature defined here
...
7 |     let result = names.iter().enumerate().map(score);
  |                                           --- ^^^^^ expected due to this
  |                                           |
  |                                           required by a bound introduced by this call
  |
  = note: expected function signature `fn((_, &&str)) -> _`
             found function signature `for<'a> fn((_, &'a str)) -> _`
note: required by a bound in `map`
 --> /rustc/5cdb7886a5ece816864fab177f0c266ad4dd5358/library/core/src/iter/traits/iterator.rs:801:5

For more information about this error, try `rustc --explain E0631`.
error: could not compile `ice-110447` (bin "ice-110447") due to previous error
