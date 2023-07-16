plain
    Checking rustc_attr v0.0.0 (/checkout/compiler/rustc_attr)
    Checking rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
    Checking rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
error[E0599]: no associated item named `STMT_EXPRPAr` found for struct `Restrictions` in the current scope
     |
     |
3380 |               let trailing = if this.restrictions.contains(Restrictions::STMT_EXPRPAr)
     |                                                                          |
     |                                                                          associated item not found in `Restrictions`
     |                                                                          help: there is an associated constant with a similar name: `STMT_EXPR`
     |
     |
    ::: /cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs:364:9
     |
364  | /         $vis struct $BitFlags {
365  | |             bits: $T,
366  | |         }
     | |_________- associated item `STMT_EXPRPAr` not found for this struct
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_parse` (lib test) due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_parse` (lib) due to previous error
