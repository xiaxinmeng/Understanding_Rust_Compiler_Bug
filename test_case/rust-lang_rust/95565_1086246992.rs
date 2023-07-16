plain
    Checking askama_shared v0.12.0
   Compiling askama_derive v0.11.0
    Checking askama v0.11.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0478]: lifetime bound not satisfied
   --> src/librustdoc/clean/auto_trait.rs:300:58
    |
300 |                     .map(|region| GenericBound::Outlives(Self::get_lifetime(*region, names_map)))
    |
    |
note: lifetime parameter instantiated with the lifetime `'tcx` as defined here
   --> src/librustdoc/clean/auto_trait.rs:27:10
    |
27  | impl<'a, 'tcx> AutoTraitFinder<'a, 'tcx> {
    |          ^^^^
note: but lifetime parameter must outlive the lifetime `'a` as defined here
   --> src/librustdoc/clean/auto_trait.rs:27:6
    |
27  | impl<'a, 'tcx> AutoTraitFinder<'a, 'tcx> {

error[E0495]: cannot infer an appropriate lifetime due to conflicting requirements
  --> src/librustdoc/html/render/write_shared.rs:70:15
   |
   |
70 |         match self {
   |               ^^^^
   |
note: first, the lifetime cannot outlive the lifetime `'_` as defined here...
  --> src/librustdoc/html/render/write_shared.rs:67:21
67 | impl SharedResource<'_> {
   |                     ^^
   |                     ^^
note: ...so that the types are compatible
  --> src/librustdoc/html/render/write_shared.rs:70:15
70 |         match self {
   |               ^^^^
   |               ^^^^
   = note: expected `&SharedResource<'_>`
              found `&SharedResource<'_>`
   = note: but, the lifetime must be valid for the static lifetime...
note: ...so that the types are compatible
  --> src/librustdoc/html/render/write_shared.rs:73:46
   |
73 |             | InvocationSpecific { basename: name } => Path::new(name).extension(),
   |                                              ^^^^
   = note: expected `&&'static str`
              found `&&str`
Some errors have detailed explanations: E0478, E0495.
For more information about an error, try `rustc --explain E0478`.
error: could not compile `rustdoc` due to 2 previous errors
Build completed unsuccessfully in 0:02:31
