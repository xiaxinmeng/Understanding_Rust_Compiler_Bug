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
    Checking parking_lot v0.12.1
    Checking crossbeam-deque v0.8.1
    Checking parking_lot v0.11.2
    Checking perf-event v0.4.7
    Checking xtask v0.1.0 (/checkout/src/tools/rust-analyzer/xtask)
    Checking regex-automata v0.1.10
    Checking regex v1.5.6
    Checking notify v5.0.0-pre.15
    Checking text-edit v0.0.0 (/checkout/src/tools/rust-analyzer/crates/text-edit)
---
    Checking proc-macro-srv v0.0.0 (/checkout/src/tools/rust-analyzer/crates/proc-macro-srv)
error[E0609]: no field `symbol_interner` on type `&mut Rustc`
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:409:60
    |
409 |                     bridge::TokenTree::Ident(SymbolId(self.symbol_interner.intern(&SymbolData(ident))))

error[E0308]: mismatched types
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:409:95
    |
    |
409 |                     bridge::TokenTree::Ident(SymbolId(self.symbol_interner.intern(&SymbolData(ident))))
    |                                                                                    ---------- ^^^^^ expected struct `SmolStr`, found struct `tt::Ident`
    |                                                                                    arguments to this struct are incorrect
    |
note: tuple struct defined here
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:133:8
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:133:8
    |
133 | struct SymbolData(tt::SmolStr);

error[E0308]: mismatched types
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:409:46
    |
    |
409 |                     bridge::TokenTree::Ident(SymbolId(self.symbol_interner.intern(&SymbolData(ident))))
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
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:411:91
    |
    |
411 |                 tt::TokenTree::Leaf(tt::Leaf::Literal(lit)) => bridge::TokenTree::Literal(lit),
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

Some errors have detailed explanations: E0308, E0609.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `proc-macro-srv` due to 4 previous errors
