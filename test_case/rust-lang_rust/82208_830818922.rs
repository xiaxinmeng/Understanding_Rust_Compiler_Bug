plain
   Compiling structopt-derive v0.4.9
    Checking thiserror v1.0.20
    Checking structopt v0.3.16
    Checking rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
error[E0046]: not all trait items implemented, missing: `SUPPORTS_CUSTOM_INNER_ATTRS`
   |
   |
57 | impl<'a> AstLike for Module<'a> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `SUPPORTS_CUSTOM_INNER_ATTRS` in implementation
   |
   = help: implement the missing item: `const SUPPORTS_CUSTOM_INNER_ATTRS: bool = true;`

error[E0277]: the trait bound `TokenStream: CreateTokenStream` is not satisfied
     |
     |
1215 |             tokens: Some(LazyTokenStream::new(ts)),
     |                                               ^^ the trait `CreateTokenStream` is not implemented for `TokenStream`
    ::: /checkout/compiler/rustc_ast/src/tokenstream.rs:143:28
     |
     |
143  |     pub fn new(inner: impl CreateTokenStream + 'static) -> LazyTokenStream {
     |                            ----------------- required by this bound in `LazyTokenStream::new`

error[E0599]: no method named `joint` found for enum `TokenTree` in the current scope
     |
     |
1262 |         let args = tok.joint();
     |                        ^^^^^ method not found in `TokenTree`
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0046, E0277, E0599.
For more information about an error, try `rustc --explain E0046`.
