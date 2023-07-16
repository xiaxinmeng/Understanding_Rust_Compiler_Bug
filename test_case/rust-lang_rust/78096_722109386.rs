
error: `f32` is both a module and a builtin type
   --> library/std/src/primitive_docs.rs:834:12
    |
834 | /// `f32`](f32) or [Wikipedia on double precision
    |            ^^^ ambiguous link
    |
    = note: `-D broken-intra-doc-links` implied by `-D warnings`
help: to link to the module, prefix with `mod@`
    |
834 | /// `f32`](mod@f32) or [Wikipedia on double precision
    |            ^^^^^^^
help: to link to the builtin type, prefix with `prim@`
    |
834 | /// `f32`](prim@f32) or [Wikipedia on double precision
    |            ^^^^^^^^

error: `f32` is both a module and a builtin type
   --> library/std/src/primitive_docs.rs:834:12
    |
834 | /// `f32`](f32) or [Wikipedia on double precision
    |            ^^^ ambiguous link
    |
help: to link to the module, prefix with `mod@`
    |
834 | /// `f32`](mod@f32) or [Wikipedia on double precision
    |            ^^^^^^^
help: to link to the builtin type, prefix with `prim@`
    |
834 | /// `f32`](prim@f32) or [Wikipedia on double precision
    |            ^^^^^^^^

error: aborting due to 2 previous errors

error: could not document `std`
