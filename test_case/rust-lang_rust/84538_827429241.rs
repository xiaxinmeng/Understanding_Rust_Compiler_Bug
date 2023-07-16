
error[E0046]: not all trait items implemented, missing: `SUPPORTS_CUSTOM_INNER_ATTRS`
   --> src/formatting/modules.rs:102:1
    |
102 | impl<'a> AstLike for Module<'a> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `SUPPORTS_CUSTOM_INNER_ATTRS` in implementation
    |
    = help: implement the missing item: `const SUPPORTS_CUSTOM_INNER_ATTRS: bool = true;`

error[E0277]: the trait bound `TokenStream: CreateTokenStream` is not satisfied
    --> src/formatting/macros.rs:1236:47
     |
1236 |             tokens: Some(LazyTokenStream::new(ts)),
     |                                               ^^ the trait `CreateTokenStream` is not implemented for `TokenStream`
     | 
    ::: /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_ast-717.0.0/src/tokenstream.rs:143:28
     |
143  |     pub fn new(inner: impl CreateTokenStream + 'static) -> LazyTokenStream {
     |                            ----------------- required by this bound in `LazyTokenStream::new`

error[E0599]: no method named `joint` found for enum `TokenTree` in the current scope
    --> src/formatting/macros.rs:1287:24
     |
1287 |         let args = tok.joint();
     |                        ^^^^^ method not found in `TokenTree`
