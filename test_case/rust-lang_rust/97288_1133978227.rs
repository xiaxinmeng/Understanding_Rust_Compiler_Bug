plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0621]: explicit lifetime required in the type of `generics`
   --> src/librustdoc/clean/mod.rs:512:36
    |
475 |     generics: Option<&hir::Generics<'tcx>>,
    |               ---------------------------- help: add explicit lifetime `'tcx` to the type of `generics`: `std::option::Option<&'tcx rustc_hir::Generics<'tcx>>`
...
512 |                     .flat_map(|bp| bp.bounds)
    |                                    ^^^^^^^^^ lifetime `'tcx` required
error: lifetime may not live long enough
    --> src/librustdoc/clean/mod.rs:1848:26
     |
     |
1842 | impl<'tcx> Clean<'tcx, GenericArgs> for hir::GenericArgs<'tcx> {
     |      ---- lifetime `'tcx` defined here
1843 |     fn clean(&self, cx: &mut DocContext<'tcx>) -> GenericArgs {
     |              - let's call the lifetime of this reference `'1`
...
1848 |             let inputs = self.inputs().iter().map(|x| x.clean(cx)).collect();
     |                          ^^^^^^^^^^^^^ argument requires that `'1` must outlive `'tcx`
For more information about this error, try `rustc --explain E0621`.
error: could not compile `rustdoc` due to 2 previous errors
Build completed unsuccessfully in 0:02:55
