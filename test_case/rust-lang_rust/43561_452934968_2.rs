rust
fn infer_using_trait_obj<'a>(t: &'a (dyn Bar + 'a)) {
    inferred_poly(t);
}
