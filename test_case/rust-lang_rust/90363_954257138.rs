
src/librustdoc/clean/utils.rs
194:                inline::build_impl(cx, None, did, None, ret);

src/librustdoc/clean/inline.rs
295:        build_impl(cx, parent_module, did, attrs, ret);

src/librustdoc/passes/collect_trait_impls.rs
35:                inline::build_impl(cx, None, did, None, &mut new_items);
44:                inline::build_impl(cx, None, def_id, None, &mut new_items);
117:                inline::build_impl(cx, None, impl_did, Some(&extra_attrs), &mut new_items);
