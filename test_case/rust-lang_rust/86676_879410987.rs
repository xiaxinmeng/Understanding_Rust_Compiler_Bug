plain
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0609]: no field `cstore` on type `context::TyCtxt<'tcx>`
   --> compiler/rustc_middle/src/ty/query/on_disk_cache.rs:830:25
    |
830 |             decoder.tcx.cstore.expn_hash_to_expn_id(krate, index_guess, hash)
    |
    |
help: one of the expressions' fields has a field of the same name
    |
830 |             decoder.tcx.gcx.untracked_resolutions.cstore.expn_hash_to_expn_id(krate, index_guess, hash)

error: aborting due to previous error

For more information about this error, try `rustc --explain E0609`.
