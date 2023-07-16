
// revisions: min_tait full_tait in_bindings
#![feature(min_type_alias_impl_trait, rustc_attrs)]
#![cfg_attr(full_tait, feature(type_alias_impl_trait))]
//[full_tait]~^ WARN incomplete
#![cfg_attr(in_bindings, feature(impl_trait_in_bindings))]
//[in_bindings]~^ WARN incomplete
