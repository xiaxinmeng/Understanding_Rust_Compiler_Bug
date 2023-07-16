
error: lifetime may not live long enough
    --> compiler/rustc_middle/src/ty/context.rs:1016:23
     |
1013 | impl<'tcx> Drop for GlobalCtxt<'tcx> {
     |      ---- lifetime `'tcx` defined here
1014 |     fn drop(&mut self) {
     |             - let's call the lifetime of this reference `'1`
1016 |         TyCtxt { gcx: self }.alloc_self_profile_query_strings();
     |                       ^^^^ requires that `'1` must outlive `'tcx`
