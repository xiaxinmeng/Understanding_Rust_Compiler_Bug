plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0061]: this function takes 4 arguments but 3 arguments were supplied
   --> src/librustdoc/passes/collect_intra_doc_links.rs:921:13
    |
921 |           tcx.find_map_relevant_impl(trait_, ty, |impl_| {
    |  _____________^^^^^^^^^^^^^^^^^^^^^^_------__--__-
    | |             expected 4 arguments
    | |             expected 4 arguments
922 | |             let trait_ref = tcx.impl_trait_ref(impl_).expect("this is not an inherent impl");
923 | |             // Check if these are the same type.
924 | |             let impl_type = trait_ref.self_ty();
...   |
945 | |             if saw_impl { Some((impl_, trait_)) } else { None }
    | |_________- supplied 3 arguments
    |
note: associated function defined here
   --> /checkout/compiler/rustc_middle/src/ty/trait_def.rs:165:12
   --> /checkout/compiler/rustc_middle/src/ty/trait_def.rs:165:12
    |
165 |     pub fn find_map_relevant_impl<T, F: FnMut(DefId) -> Option<T>>(

For more information about this error, try `rustc --explain E0061`.
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:03:11
