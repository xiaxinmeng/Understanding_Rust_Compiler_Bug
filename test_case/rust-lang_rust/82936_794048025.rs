plain
    Checking cranelift-frontend v0.70.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#cdb60ec5)
    Checking cranelift-jit v0.70.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#cdb60ec5)
    Checking cranelift-object v0.70.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#cdb60ec5)
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0609]: no field `val` on type `ConstantSource<'_>`
   |
   |
43 |         match const_.val {


error[E0609]: no field `val` on type `ConstantSource<'_>`
    |
    |
117 |     let const_val = match const_.val {


error[E0615]: attempted to take value of method `ty` on type `ConstantSource<'tcx>`
    |
    |
126 |                 fx.layout_of(fx.monomorphize(&constant.literal.ty)),
    |                                                                ^^ method, not a field
help: use parentheses to call the method
    |
    |
126 |                 fx.layout_of(fx.monomorphize(&constant.literal.ty())),


error[E0615]: attempted to take value of method `ty` on type `ConstantSource<'_>`
    |
    |
145 |     codegen_const_value(fx, const_val, const_.ty)
    |                                               ^^ method, not a field
help: use parentheses to call the method
    |
    |
145 |     codegen_const_value(fx, const_val, const_.ty())


error[E0599]: no method named `eval` found for enum `ConstantSource<'_>` in the current scope
    |
    |
429 |             Some(fx.monomorphize(const_.literal).eval(fx.tcx, ParamEnv::reveal_all()))
    |                                                  ^^^^ method not found in `ConstantSource<'_>`
warning: unused import: `cranelift_codegen::entity::EntityRef`
 --> src/value_and_place.rs:5:5
  |
5 | use cranelift_codegen::entity::EntityRef;
