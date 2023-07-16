plain
    Checking cranelift-frontend v0.76.0 (https://github.com/bytecodealliance/wasmtime.git#9c550fcf)
    Checking cranelift-native v0.76.0 (https://github.com/bytecodealliance/wasmtime.git#9c550fcf)
    Checking cranelift-object v0.76.0 (https://github.com/bytecodealliance/wasmtime.git#9c550fcf)
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0616]: field `val` of struct `rustc_middle::ty::Const` is private
    |
664 |                         .val
    |                          ^^^ private field
    |
    |
help: a method `val` also exists, call it with parentheses
    |
664 |                         .val()
    |                             ++

error[E0616]: field `val` of struct `rustc_middle::ty::Const` is private
   |
49 |         match const_.val {
   |                      ^^^ private field
   |
   |
help: a method `val` also exists, call it with parentheses
   |
49 |         match const_.val() {


error[E0616]: field `val` of struct `rustc_middle::ty::Const` is private
    |
130 |     let const_val = match const_.val {
    |                                  ^^^ private field
    |
    |
help: a method `val` also exists, call it with parentheses
    |
130 |     let const_val = match const_.val() {


error[E0616]: field `ty` of struct `rustc_middle::ty::Const` is private
    |
    |
136 |             return codegen_static_ref(fx, uv.def.did, fx.layout_of(const_.ty)).to_cvalue(fx);
    |                                                                           ^^ private field
    |
help: a method `ty` also exists, call it with parentheses
    |
136 |             return codegen_static_ref(fx, uv.def.did, fx.layout_of(const_.ty())).to_cvalue(fx);


error[E0616]: field `ty` of struct `rustc_middle::ty::Const` is private
    |
    |
153 |     codegen_const_value(fx, const_val, const_.ty)
    |                                               ^^ private field
    |
help: a method `ty` also exists, call it with parentheses
    |
153 |     codegen_const_value(fx, const_val, const_.ty())


error[E0616]: field `val` of struct `rustc_middle::ty::Const` is private
    |
    |
465 |                 fx.monomorphize(const_).eval(fx.tcx, ParamEnv::reveal_all()).val.try_to_value()
    |
help: a method `val` also exists, call it with parentheses
    |
    |
465 |                 fx.monomorphize(const_).eval(fx.tcx, ParamEnv::reveal_all()).val().try_to_value()

For more information about this error, try `rustc --explain E0616`.
error: could not compile `rustc_codegen_cranelift` due to 6 previous errors
Build completed unsuccessfully in 0:03:43
