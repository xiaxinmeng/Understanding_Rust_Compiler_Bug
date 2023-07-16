plain
   Compiling rustc_attr v0.0.0 (/checkout/compiler/rustc_attr)
   Compiling rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
   Compiling rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
error[E0277]: the trait bound `PackedDelimTokenVec: Clone` is not satisfied
    |
351 |   #[derive(Clone, Default)]
    |            ----- in this macro invocation
...
...
355 |       delims: PackedDelimTokenVec,
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `PackedDelimTokenVec`
   ::: /checkout/library/core/src/clone.rs:139:1
    |
    |
139 | / pub macro Clone($item:item) {
141 | | }
141 | | }
    | |_- in this expansion of `#[derive(Clone)]`
    = note: required by `std::clone::Clone::clone`


error[E0277]: the trait bound `PackedDelimFlags: Clone` is not satisfied
    |
351 |   #[derive(Clone, Default)]
    |            ----- in this macro invocation
...
...
358 |       delim_flags: PackedDelimFlags,
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `PackedDelimFlags`
   ::: /checkout/library/core/src/clone.rs:139:1
    |
    |
139 | / pub macro Clone($item:item) {
141 | | }
141 | | }
    | |_- in this expansion of `#[derive(Clone)]`
    = note: required by `std::clone::Clone::clone`

error: aborting due to 2 previous errors

