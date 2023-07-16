plain
     |               ^^^^^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `has_impossible_predicates`
     |
    ::: /checkout/compiler/rustc_trait_selection/src/traits/mod.rs:428:1
     |
428  | / fn has_impossible_predicates<'tcx>(
429  | |     tcx: TyCtxt<'tcx>,
430  | |     predicates: impl Iterator<Item = ty::Predicate<'tcx>>,
431  | | ) -> bool {
     | |_________- similarly named function `has_impossible_predicates` defined here
   Compiling libz-sys v1.1.3
    Checking pulldown-cmark v0.9.1
    Checking aho-corasick v0.7.18
    Checking bstr v0.2.13
