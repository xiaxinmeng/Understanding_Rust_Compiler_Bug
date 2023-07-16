
error[E0277]: expected a `std::ops::Fn<(syn::lookahead::TokenMarker,)>` closure, found `impl Peek<Token = T>`
  --> src/main.rs:20:82
   |
17 |     ending_tokens: &[&impl Peek<Token = T>],
   |                       -------------------- help: consider further restricting this bound: `impl Peek<Token = T> + std::ops::Fn<(syn::lookahead::TokenMarker,)>`
...
20 |     while !input.is_empty() && !ending_tokens.into_iter().any(|token| input.peek(token)) {
   |                                                                                  ^^^^^ expected an `Fn<(syn::lookahead::TokenMarker,)>` closure, found `impl Peek<Token = T>`
   |
   = help: the trait `std::ops::Fn<(syn::lookahead::TokenMarker,)>` is not implemented for `impl Peek<Token = T>`
   = note: required because of the requirements on the impl of `std::ops::FnOnce<(syn::lookahead::TokenMarker,)>` for `&impl Peek<Token = T>`
   = note: required because of the requirements on the impl of `syn::lookahead::Peek` for `&&impl Peek<Token = T>`
