
  Compiling playground v0.0.1 (/playground)
error[E0277]: expected a `std::ops::Fn<(syn::lookahead::TokenMarker,)>` closure, found `impl Peek<Token = T>`
  --> src/main.rs:20:82
   |
20 |     while !input.is_empty() && !ending_tokens.into_iter().any(|token| input.peek(token)) {
   |                                                                                  ^^^^^ expected an `Fn<(syn::lookahead::TokenMarker,)>` closure, found `impl Peek<Token = T>`
   |
   = note: required because of the requirements on the impl of `FnOnce<(syn::lookahead::TokenMarker,)>` for `&impl Peek<Token = T>`
   = note: required because of the requirements on the impl of `Peek` for `&&impl Peek<Token = T>`
help: consider further restricting this bound
   |
17 |     ending_tokens: &[&impl Peek<Token = T> + std::ops::Fn<(syn::lookahead::TokenMarker,)>],
   |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
error: could not compile `playground`

To learn more, run the command again with --verbose.
