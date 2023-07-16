plain
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
   --> src/value_and_place.rs:818:23
    |
818 |         (&ty::Dynamic(from_traits, _), &ty::Dynamic(to_traits, _)) => {
    |
    |
   ::: /checkout/compiler/rustc_type_ir/src/sty.rs:119:13
    |
119 |     Dynamic(I::ListBinderExistentialPredicate, I::Region, DynKind),
    |
help: use `_` to explicitly ignore each field
    |
    |
818 |         (&ty::Dynamic(from_traits, _, _), &ty::Dynamic(to_traits, _)) => {

error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
   --> src/value_and_place.rs:818:53
    |
    |
818 |         (&ty::Dynamic(from_traits, _), &ty::Dynamic(to_traits, _)) => {
    |
    |
   ::: /checkout/compiler/rustc_type_ir/src/sty.rs:119:13
    |
119 |     Dynamic(I::ListBinderExistentialPredicate, I::Region, DynKind),
    |
help: use `_` to explicitly ignore each field
    |
    |
818 |         (&ty::Dynamic(from_traits, _), &ty::Dynamic(to_traits, _, _)) => {

For more information about this error, try `rustc --explain E0023`.
error: could not compile `rustc_codegen_cranelift` due to 2 previous errors
Build completed unsuccessfully in 0:03:32
