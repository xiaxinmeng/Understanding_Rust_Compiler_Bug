plain
    Checking rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
    Checking rustc_arena v0.0.0 (/checkout/compiler/rustc_arena)
    Checking rustc_type_ir v0.0.0 (/checkout/compiler/rustc_type_ir)
    Checking rustc_span v0.0.0 (/checkout/compiler/rustc_span)
error: Symbol `HashMapEntry` must precede `Hasher`
    |
177 |         HashMapEntry,
    |         ^^^^^^^^^^^^


error: location of previous symbol `Hasher`
    |
176 |         Hasher,
    |         ^^^^^^

