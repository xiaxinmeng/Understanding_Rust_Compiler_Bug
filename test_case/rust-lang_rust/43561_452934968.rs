rust
23 | fn infer_using_trait_obj(t: &dyn Bar) {
   |                             -------- help: add explicit lifetime `'static` to the type of `t`: `&'static (dyn Bar + 'static)`
24 |     inferred_poly(t);
   |     ^^^^^^^^^^^^^ lifetime `'static` required
