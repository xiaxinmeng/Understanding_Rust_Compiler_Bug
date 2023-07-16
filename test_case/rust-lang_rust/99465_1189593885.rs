plain
    Checking lock_api v0.4.7
    Checking always-assert v0.1.2
    Checking ena v0.14.0
    Checking tracing-log v0.1.3
    Checking sourcegen v0.0.0 (/checkout/src/tools/rust-analyzer/crates/sourcegen)
    Checking flate2 v1.0.24
    Checking xflags v0.2.4
    Checking parking_lot_core v0.9.3
    Checking stdx v0.0.0 (/checkout/src/tools/rust-analyzer/crates/stdx)
---
    Checking perf-event v0.4.7
    Checking parking_lot v0.11.2
    Checking regex-automata v0.1.10
    Checking regex v1.5.6
    Checking xtask v0.1.0 (/checkout/src/tools/rust-analyzer/xtask)
    Checking pulldown-cmark v0.9.1
    Checking notify v5.0.0-pre.15
    Checking countme v3.0.1
    Checking profile v0.0.0 (/checkout/src/tools/rust-analyzer/crates/profile)
---
    |
615 | impl server::Span for Rustc {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `subspan` in implementation
    |
    = help: implement the missing item: `fn subspan(&mut self, _: <Self as Types>::Span, _: Bound<usize>, _: Bound<usize>) -> Option<<Self as Types>::Span> { todo!() }`
error[E0308]: mismatched types
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:326:38
    |
316 |         match tree {
316 |         match tree {
    |               ---- this expression has type `proc_macro::bridge::TokenTree<rustc_server::TokenStream, TokenId, SymbolId>`
...
326 |             bridge::TokenTree::Ident(SymbolId(index)) => {
    |                                      ^^^^^^^^^^^^^^^ expected struct `proc_macro::bridge::Ident`, found struct `SymbolId`
    |
    = note: expected struct `proc_macro::bridge::Ident<TokenId, SymbolId>`
               found struct `SymbolId`
error[E0308]: mismatched types
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:328:40
    |
    |
328 |                 let ident: tt::Ident = ident;
    |                            ---------   ^^^^^ expected struct `tt::Ident`, found struct `SmolStr`
    |                            expected due to this


error[E0277]: the trait bound `tt::Leaf: From<proc_macro::bridge::Literal<TokenId, SymbolId>>` is not satisfied
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:335:28
    |
335 |                 let leaf = tt::Leaf::from(literal);
    |                            ^^^^^^^^^^^^^^ the trait `From<proc_macro::bridge::Literal<TokenId, SymbolId>>` is not implemented for `tt::Leaf`
    |
    = help: the following other types implement trait `From<T>`:
              <tt::Leaf as From<tt::Ident>>
              <tt::Leaf as From<tt::Literal>>
              <tt::Leaf as From<tt::Punct>>
error[E0308]: mismatched types
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:395:95
    |
    |
395 |                     bridge::TokenTree::Ident(SymbolId(self.symbol_interner.intern(&SymbolData(ident))))
    |                                                                                    ---------- ^^^^^ expected struct `SmolStr`, found struct `tt::Ident`
    |                                                                                    arguments to this struct are incorrect
    |
note: tuple struct defined here
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:132:8
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:132:8
    |
132 | struct SymbolData(tt::SmolStr);

error[E0308]: mismatched types
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:395:46
    |
    |
395 |                     bridge::TokenTree::Ident(SymbolId(self.symbol_interner.intern(&SymbolData(ident))))
    |                     ------------------------ ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `proc_macro::bridge::Ident`, found struct `SymbolId`
    |                     arguments to this enum variant are incorrect
    |
    = note: expected struct `proc_macro::bridge::Ident<_, _>`
               found struct `SymbolId`
               found struct `SymbolId`
note: tuple variant defined here
   --> /checkout/library/proc_macro/src/bridge/mod.rs:500:5
    |
500 |     Ident(Ident<Span, Symbol>),

error[E0308]: mismatched types
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:397:91
    |
    |
397 |                 tt::TokenTree::Leaf(tt::Leaf::Literal(lit)) => bridge::TokenTree::Literal(lit),
    |                                                                -------------------------- ^^^ expected struct `proc_macro::bridge::Literal`, found struct `tt::Literal`
    |                                                                arguments to this enum variant are incorrect
    |
    = note: expected struct `proc_macro::bridge::Literal<_, _>`
    = note: expected struct `proc_macro::bridge::Literal<_, _>`
               found struct `tt::Literal`
   --> /checkout/library/proc_macro/src/bridge/mod.rs:501:5
    |
    |
501 |     Literal(Literal<Span, Symbol>),

Some errors have detailed explanations: E0046, E0277, E0308.
For more information about an error, try `rustc --explain E0046`.
error: could not compile `proc-macro-srv` due to 7 previous errors
