plain
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error[E0599]: no method named `as_str` found for reference `&Cow<'_, str>` in the current scope
    --> compiler/rustc_codegen_ssa/src/back/link.rs:2611:25
     |
2611 |         || !matches!(os.as_str(), "ios" | "tvos" | "watchos")

    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_codegen_ssa` due to previous error
