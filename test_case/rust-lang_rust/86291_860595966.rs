plain
    Checking cranelift-frontend v0.74.0 (https://github.com/bytecodealliance/wasmtime.git?branch=main#76c6b83f)
    Checking cranelift-object v0.74.0 (https://github.com/bytecodealliance/wasmtime.git?branch=main#76c6b83f)
    Checking cranelift-jit v0.74.0 (https://github.com/bytecodealliance/wasmtime.git?branch=main#76c6b83f)
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0599]: no method named `vtable_methods` found for struct `rustc_middle::ty::TyCtxt<'tcx>` in the current scope
  --> src/vtable.rs:99:13
   |
99 |         tcx.vtable_methods(trait_ref.with_self_ty(tcx, layout.ty));
   |             ^^^^^^^^^^^^^^ method not found in `rustc_middle::ty::TyCtxt<'tcx>`
error[E0614]: type `rustc_span::def_id::DefId` cannot be dereferenced
   --> src/vtable.rs:133:79
    |
    |
133 |                     Instance::resolve_for_vtable(tcx, ParamEnv::reveal_all(), *def_id, substs)

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0599, E0614.
