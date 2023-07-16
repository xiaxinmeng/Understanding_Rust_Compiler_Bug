
error: lifetime may not live long enough
   --> src/librustdoc/clean/auto_trait.rs:144:9
    |
27  | impl<'a, 'tcx> AutoTraitFinder<'a, 'tcx> {
    |      --  ---- lifetime `'tcx` defined here
    |      |
    |      lifetime `'a` defined here
...
144 |         Self::region_name(region)
    |         ^^^^^^^^^^^^^^^^^ requires that `'tcx` must outlive `'a`
    |
    = help: consider adding the following bound: `'tcx: 'a`

error: lifetime may not live long enough
   --> src/librustdoc/clean/auto_trait.rs:202:24
    |
27  | impl<'a, 'tcx> AutoTraitFinder<'a, 'tcx> {
    |      --  ---- lifetime `'tcx` defined here
    |      |
    |      lifetime `'a` defined here
...
202 |                     if Self::region_name(r1) != Self::region_name(r2) {
    |                        ^^^^^^^^^^^^^^^^^ requires that `'tcx` must outlive `'a`
    |
    = help: consider adding the following bound: `'tcx: 'a`

error: aborting due to 2 previous errors
