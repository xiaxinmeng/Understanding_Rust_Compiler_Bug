plain
    Checking cranelift-native v0.83.0
    Checking cranelift-frontend v0.83.0
    Checking cranelift-object v0.83.0
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0599]: no method named `needs_infer` found for reference `&'tcx rustc_middle::ty::List<rustc_middle::ty::GenericArg<'tcx>>` in the current scope
   --> src/abi/mod.rs:56:26
    |
56  |     assert!(!inst.substs.needs_infer());
    |                          ^^^^^^^^^^^ method not found in `&'tcx rustc_middle::ty::List<rustc_middle::ty::GenericArg<'tcx>>`
   ::: /checkout/compiler/rustc_middle/src/ty/visit.rs:119:8
    |
119 |     fn needs_infer(&self) -> bool {
119 |     fn needs_infer(&self) -> bool {
    |        ----------- the method is available for `&'tcx rustc_middle::ty::List<rustc_middle::ty::GenericArg<'tcx>>` here
    = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
    |
3   | use crate::rustc_middle::ty::TypeVisitable;
3   | use crate::rustc_middle::ty::TypeVisitable;
    |

error[E0599]: no method named `needs_infer` found for reference `&'tcx rustc_middle::ty::List<rustc_middle::ty::GenericArg<'tcx>>` in the current scope
    |
    |
24  |     debug_assert!(!instance.substs.needs_infer());
    |                                    ^^^^^^^^^^^ method not found in `&'tcx rustc_middle::ty::List<rustc_middle::ty::GenericArg<'tcx>>`
   ::: /checkout/compiler/rustc_middle/src/ty/visit.rs:119:8
    |
119 |     fn needs_infer(&self) -> bool {
119 |     fn needs_infer(&self) -> bool {
    |        ----------- the method is available for `&'tcx rustc_middle::ty::List<rustc_middle::ty::GenericArg<'tcx>>` here
    = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
    |
3   | use crate::rustc_middle::ty::TypeVisitable;
