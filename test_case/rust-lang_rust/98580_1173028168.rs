plain

   Doc-tests rustc_lint_defs

running 110 tests
ii..i..ii......i......i..ii.i.......i......iii.....................i...F....i..i........ 88/110
failures:

---- src/builtin.rs - builtin::NAMED_ARGUMENTS_USED_POSITIONALLY (line 3945) stdout ----
warning: unknown lint: `named_arguments_used_positionally`
warning: unknown lint: `named_arguments_used_positionally`
 --> src/builtin.rs:3946:9
  |
2 | #![deny(named_arguments_used_positionally)]
  |
  = note: `#[warn(unknown_lints)]` on by default


warning: named argument _x is not used by name
  |
  |
5 |     println!("{}", _x = 1); // Prints 1, will trigger lint
  |               --   ^^ this named argument is only referred to by position in formatting string
  |               |
  |               this formatting argument uses named argument _x by position
  |               help: use the named argument by name to avoid ambiguity: `{_x}`
  |
  = note: `#[warn(named_arguments_used_positionally)]` on by default
warning: 2 warnings emitted

Test compiled successfully, but it's marked `compile_fail`.

