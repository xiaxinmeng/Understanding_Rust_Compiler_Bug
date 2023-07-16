
error[E0593]: function is expected to take 1 argument, but it takes 2 arguments
 --> main.rs:2:7
  |
2 |     s.contains(char::is_digit)
  |       ^^^^^^^^ expected function that takes 1 argument
  |
  = note: required because of the requirements on the impl of `std::str::pattern::Pattern<'_>` for `fn(char, u32) -> bool {std::char::<impl char>::is_digit}`

error: aborting due to previous error
