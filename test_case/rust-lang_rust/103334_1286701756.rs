plain
    Checking object v0.26.2
    Checking hashbrown v0.12.3
    Checking miniz_oxide v0.4.0
    Checking addr2line v0.16.0
error: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
    |
    |
365 | #[feature(pdbg)]
    |
    |
    = note: `-D unused-attributes` implied by `-D warnings`

error: duplicate diagnostic item found: `dbg_macro`.
    |
366 | macro_rules! pdbg {
    | ^^^^^^^^^^^^^^^^^
    |
