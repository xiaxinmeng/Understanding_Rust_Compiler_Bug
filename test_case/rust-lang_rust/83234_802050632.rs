plain
    Checking cranelift-frontend v0.70.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#cdb60ec5)
    Checking cranelift-jit v0.70.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#cdb60ec5)
    Checking cranelift-object v0.70.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#cdb60ec5)
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0308]: `match` arms have incompatible types
    |
    |
123 |       let const_val = match const_.val {
    |                       ---------------- `match` arms have incompatible types
124 |           ConstKind::Value(const_val) => const_val,
    |                                          --------- this is found to be of type `ValTree<'_>`
...
129 |               return codegen_static_ref(fx, def.did, fx.layout_of(const_.ty)).to_cvalue(fx);
    |               ------------------------------------------------------------------------------ this is found to be of type `ValTree<'_>`
...
132 | /             match fx.tcx.const_eval_resolve(ParamEnv::reveal_all(), def, substs, promoted, None) {
133 | |                 Ok(const_val) => const_val,
134 | |                 Err(_) => {
135 | |                     span_bug!(constant.span, "erroneous constant not captured by required_consts");
137 | |             }
137 | |             }
    | |_____________^ expected enum `ValTree`, found enum `rustc_middle::mir::interpret::ConstValue`
error[E0308]: mismatched types
   --> src/constant.rs:431:17
    |
    |
431 |                 fx.monomorphize(const_).eval(fx.tcx, ParamEnv::reveal_all()).val.try_to_value()
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_middle::mir::interpret::ConstValue`, found enum `ValTree`
    = note: expected enum `Option<rustc_middle::mir::interpret::ConstValue<'tcx>>`
    = note: expected enum `Option<rustc_middle::mir::interpret::ConstValue<'tcx>>`
               found enum `Option<ValTree<'_>>`
warning: unused import: `cranelift_codegen::entity::EntityRef`
 --> src/value_and_place.rs:5:5
  |
5 | use cranelift_codegen::entity::EntityRef;
