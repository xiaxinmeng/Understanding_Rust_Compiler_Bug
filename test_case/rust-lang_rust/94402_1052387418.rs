plain
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
error[E0599]: no method named `apply_attrs_to_cleanup_callsite` found for mutable reference `&mut Bx` in the current scope
    |
    |
169 |                 bx.apply_attrs_to_cleanup_callsite(invokeret);
    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `&mut Bx`

error[E0599]: no method named `apply_attrs_to_cleanup_callsite` found for type parameter `Bx` in the current scope
     |
     |
1451 |             bx.apply_attrs_to_cleanup_callsite(llret);
     |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `Bx`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_codegen_ssa` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
