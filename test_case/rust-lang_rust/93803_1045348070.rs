plain
    Checking rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0609]: no field `where_clause` on type `&rustc_hir::Generics<'_>`
   --> compiler/rustc_infer/src/infer/error_reporting/note.rs:379:26
379 |                         .where_clause
    |                          ^^^^^^^^^^^^ help: a field with a similar name exists: `has_where_clause`

For more information about this error, try `rustc --explain E0609`.
