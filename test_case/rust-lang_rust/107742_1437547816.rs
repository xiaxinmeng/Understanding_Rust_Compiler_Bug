plain
    Checking rustc_const_eval v0.0.0 (/checkout/compiler/rustc_const_eval)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
error[E0107]: this struct takes 0 generic arguments but 1 generic argument was supplied
     |
     |
74   | ) -> Result<LayoutS<VariantIdx>, LayoutError<'tcx>> {
     |             ^^^^^^^------------ help: remove these generics
     |             expected 0 generic arguments
     |
note: struct defined here, with 0 generic parameters
    --> /checkout/compiler/rustc_abi/src/lib.rs:1389:12
