plain
    Checking rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
    Checking rustc_smir v0.0.0 (/checkout/compiler/rustc_smir)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error[E0609]: no field `module` on type `&&Import<'_>`
     |
1030 |             let module = match glob_import.module.get() {
     |                                            ^^^^^^ unknown field
     |
     |
     = note: available fields are: `kind`, `root_id`, `use_span`, `use_span_with_attributes`, `has_attributes` ... and 7 others
     |
     |
1030 |             let module = match glob_import.parent_scope.module.get() {

For more information about this error, try `rustc --explain E0609`.
error: could not compile `rustc_resolve` due to previous error
warning: build failed, waiting for other jobs to finish...
