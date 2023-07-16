plain
    Checking cranelift-frontend v0.74.0 (https://github.com/bytecodealliance/wasmtime.git?branch=main#76c6b83f)
    Checking cranelift-object v0.74.0 (https://github.com/bytecodealliance/wasmtime.git?branch=main#76c6b83f)
    Checking cranelift-jit v0.74.0 (https://github.com/bytecodealliance/wasmtime.git?branch=main#76c6b83f)
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0308]: `if` and `else` have incompatible types
   --> src/vtable.rs:101:9
    |
98  |       let vtable_entries = if let Some(trait_ref) = trait_ref {
    |  __________________________-
99  | |         tcx.vtable_entries(trait_ref.with_self_ty(tcx, layout.ty));
    | |         |                                                         |
    | |         |                                                         help: consider removing this semicolon
    | |         expected because of this
100 | |     } else {
100 | |     } else {
101 | |         ty::COMMON_VTABLE_ENTRIES
    | |         ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found `&[VtblEntry<'_>]`
102 | |     };
    | |_____- `if` and `else` have incompatible types
    = note:   expected type `()`
    = note:   expected type `()`
            found reference `&[VtblEntry<'_>]`
error[E0614]: type `rustc_span::def_id::DefId` cannot be dereferenced
   --> src/vtable.rs:133:79
    |
    |
133 |                     Instance::resolve_for_vtable(tcx, ParamEnv::reveal_all(), *def_id, substs)

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0308, E0614.
