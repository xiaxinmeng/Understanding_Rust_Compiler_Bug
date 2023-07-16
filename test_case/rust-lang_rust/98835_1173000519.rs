plain
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
error: lifetime may not live long enough
   --> src/librustdoc/clean/auto_trait.rs:294:23
    |
27  |   impl<'a, 'tcx> AutoTraitFinder<'a, 'tcx> {
    |        --  ---- lifetime `'tcx` defined here
    |        |
    |        lifetime `'a` defined here
...
294 |               .flat_map(|(name, lifetime)| {
    |  _______________________^
295 | |                 let empty = Vec::new();
296 | |                 let bounds: FxHashSet<GenericBound> = finished
297 | |                     .get(name)
309 | |                 })
310 | |             })
310 | |             })
    | |_____________^ requires that `'tcx` must outlive `'a`
    |
    = help: consider adding the following bound: `'tcx: 'a`
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:13:18
