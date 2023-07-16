plain
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0063]: missing field `reveal_defining_opaque_types` in initializer of `infer::InferCtxt<'a, 'tcx>`
  --> compiler/rustc_infer/src/infer/at.rs:65:9
65 |         Self {
65 |         Self {
   |         ^^^^ missing `reveal_defining_opaque_types`

error[E0277]: the trait bound `OpaqueTypeStorage<'_>: std::clone::Clone` is not satisfied
    |
133 |   #[derive(Clone)]
    |            ----- in this derive macro expansion
...
...
197 |       pub opaque_type_storage: OpaqueTypeStorage<'tcx>,
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::clone::Clone` is not implemented for `OpaqueTypeStorage<'_>`
   ::: /checkout/library/core/src/clone.rs:139:1
    |
139 | / pub macro Clone($item:item) {
140 | |     /* compiler built-in */
