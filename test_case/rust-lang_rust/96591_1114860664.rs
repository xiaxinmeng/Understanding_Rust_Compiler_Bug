plain
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0424]: expected value, found module `self`
   --> src/constant.rs:475:31
    |
465 | pub(crate) fn mir_operand_get_const_val<'tcx>(
    |               ------------------------- this function can't have a `self` parameter
...
475 |                 .try_to_value(self.tcx),
    |                               ^^^^ `self` value is a keyword only available in methods with a `self` parameter
error[E0308]: `match` arms have incompatible types
   --> src/constant.rs:140:13
    |
    |
129 |       let const_val = match const_.val() {
    |                       ------------------ `match` arms have incompatible types
130 |           ConstKind::Value(const_val) => const_val,
    |                                          --------- this is found to be of type `ValTree<'_>`
...
137 |               return codegen_static_ref(fx, def.did, fx.layout_of(const_.ty())).to_cvalue(fx);
    |               -------------------------------------------------------------------------------- this is found to be of type `ValTree<'_>`
...
140 | /             match fx.tcx.const_eval_resolve(ParamEnv::reveal_all(), unevaluated, None) {
141 | |                 Ok(const_val) => const_val,
142 | |                 Err(_) => {
143 | |                     span_bug!(constant.span, "erroneous constant not captured by required_consts");
145 | |             }
145 | |             }
    | |_____________^ expected enum `ValTree`, found enum `rustc_middle::mir::interpret::ConstValue`
error[E0061]: this function takes 0 arguments but 1 argument was supplied
   --> src/constant.rs:475:18
    |
    |
475 |                 .try_to_value(self.tcx),
    |                  |
    |                  expected 0 arguments
    |
note: associated function defined here
note: associated function defined here
   --> /checkout/compiler/rustc_middle/src/ty/consts/kind.rs:78:12
    |
78  |     pub fn try_to_value(self) -> Option<ty::ValTree<'tcx>> {

error[E0308]: mismatched types
   --> src/constant.rs:471:41
    |
    |
468 |   ) -> Option<ConstValue<'tcx>> {
    |        ------------------------ expected `Option<rustc_middle::mir::interpret::ConstValue<'tcx>>` because of return type
...
471 |               ConstantKind::Ty(const_) => fx
472 | |                 .monomorphize(const_)
472 | |                 .monomorphize(const_)
473 | |                 .eval(fx.tcx, ParamEnv::reveal_all())
474 | |                 .val()
475 | |                 .try_to_value(self.tcx),
    | |_______________________________________^ expected enum `rustc_middle::mir::interpret::ConstValue`, found enum `ValTree`
    = note: expected enum `Option<rustc_middle::mir::interpret::ConstValue<'tcx>>`
    = note: expected enum `Option<rustc_middle::mir::interpret::ConstValue<'tcx>>`
               found enum `Option<ValTree<'_>>`
Some errors have detailed explanations: E0061, E0308, E0424.
For more information about an error, try `rustc --explain E0061`.
error: could not compile `rustc_codegen_cranelift` due to 4 previous errors
Build completed unsuccessfully in 0:03:39
