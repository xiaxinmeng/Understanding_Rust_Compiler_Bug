plain
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error[E0308]: mismatched types
  --> compiler/rustc_mir_transform/src/add_retag.rs:96:36
   |
96 |                     if needs_retag(place) { Some((place, decl.source_info)) } else { None }
   |                        |           |
   |                        |           |
   |                        |           expected `&rustc_middle::mir::Place<'tcx>`, found struct `rustc_middle::mir::Place`
   |                        arguments to this function are incorrect
   |
note: closure defined here
  --> compiler/rustc_mir_transform/src/add_retag.rs:71:27
  --> compiler/rustc_mir_transform/src/add_retag.rs:71:27
   |
71 |         let needs_retag = |place: &Place<'tcx>| {

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_mir_transform` due to previous error
warning: build failed, waiting for other jobs to finish...
