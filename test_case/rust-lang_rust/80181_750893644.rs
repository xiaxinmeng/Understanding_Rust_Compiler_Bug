
 Documenting rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
[RUSTC-TIMING] rustc_parse_format test:false 0.192
[RUSTC-TIMING] rustc_feature test:false 0.383
error: unresolved link to `repr(transparent)`
   --> compiler/rustc_feature/src/active.rs:46:19
    |
46  |                   $(#[doc = $doc])*
...
...
95  | / declare_features! (
96  | |     // -------------------------------------------------------------------------
97  | |     // feature-group-start: internal feature gates
98  | |     // -------------------------------------------------------------------------
628 | |     // -------------------------------------------------------------------------
629 | | );
    | |__- in this macro invocation
    |
    |
    = note: `-D broken-intra-doc-links` implied by `-D warnings`
    = note: the link appears in this line:
            
            Allows #[repr(transparent)] on unions (RFC 2645).
                     ^^^^^^^^^^^^^^^^^
    = note: no item named `repr(transparent)` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
