
error[E0433]: failed to resolve: unresolved import
  --> <::syn::token::Token macros>:37:55
   |
1  | / ( abstract ) => { crate :: token :: Abstract } ; ( as ) => {
2  | | crate :: token :: As } ; ( async ) => { crate :: token :: Async } ; ( auto )
3  | | => { crate :: token :: Auto } ; ( become ) => { crate :: token :: Become } ; (
4  | | box ) => { crate :: token :: Box } ; ( break ) => { crate :: token :: Break }
...  |
37 | | => { crate :: token :: Colon2 } ; ( , ) => { crate :: token :: Comma } ; ( / )
   | |                                                       ^^^^^
   | |                                                       |
   | |                                                       unresolved import
   | |                                                       help: a similar path exists: `syn::token`
...  |
54 | | ) => { crate :: token :: Tilde } ; ( _ ) => { crate :: token :: Underscore } ;
55 | | ( $ ) => { crate :: token :: Dollar } ;
   | |_______________________________________- in this expansion of `Token!`
   |
  ::: serde_derive/src/fragment.rs:58:18
   |
58 |                   <Token![,]>::default().to_tokens(out);
   |                    --------- in this macro invocation

error: aborting due to 12 previous errors

For more information about this error, try `rustc --explain E0433`.
error: Could not compile `serde_derive`.
