plain
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0609]: no field `parenting` on type `&OwnerNodes<'_>`
    |
    |
137 |                 tcx.hir_owner_nodes(parent_hir_id.owner).unwrap().parenting[&id.def_id];
    |
    |
    = note: available fields are: `hash_including_bodies`, `hash_without_bodies`, `nodes`, `bodies`, `local_id_to_def_id`
For more information about this error, try `rustc --explain E0609`.
error: could not compile `rustc_middle` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_middle` due to previous error
