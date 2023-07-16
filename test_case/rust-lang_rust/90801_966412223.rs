plain
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error[E0277]: a value of type `Vec<Arc<QueryRegionConstraints<'_>>>` cannot be built from an iterator over elements of type `Rc<QueryRegionConstraints<'_>>`
   --> compiler/rustc_borrowck/src/type_check/input_output.rs:220:64
220 |         let mut arg_constraints = constraints_args.into_iter().collect();
220 |         let mut arg_constraints = constraints_args.into_iter().collect();
    |                                                                ^^^^^^^ value of type `Vec<Arc<QueryRegionConstraints<'_>>>` cannot be built from `std::iter::Iterator<Item=Rc<QueryRegionConstraints<'_>>>`
    |
    = help: the trait `FromIterator<Rc<QueryRegionConstraints<'_>>>` is not implemented for `Vec<Arc<QueryRegionConstraints<'_>>>`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_borrowck` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
