plain
[RUSTC-TIMING] rustc_passes test:false 30.301
   Compiling rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
[RUSTC-TIMING] rustc_ty_utils test:false 16.329
   Compiling rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error[E0277]: the size for values of type `[(&Candidate<'tcx>, ProbeResult)]` cannot be known at compilation time
     |
     |
1662 |         for (p, _) in probes[1..] {
     |                       ^^^^^^^^^^^ expected an implementor of trait `IntoIterator`
     |
     = note: the trait bound `[(&Candidate<'tcx>, ProbeResult)]: IntoIterator` is not satisfied
     = note: required because of the requirements on the impl of `IntoIterator` for `[(&Candidate<'tcx>, ProbeResult)]`
     |
     |
1662 |         for (p, _) in &probes[1..] {
     |                       +
1662 |         for (p, _) in &mut probes[1..] {


error[E0277]: `[(&Candidate<'tcx>, ProbeResult)]` is not an iterator
     |
     |
1662 |         for (p, _) in probes[1..] {
     |                       ^^^^^^^^^^^ expected an implementor of trait `IntoIterator`
     |
     = note: the trait bound `[(&Candidate<'tcx>, ProbeResult)]: IntoIterator` is not satisfied
     = note: required because of the requirements on the impl of `IntoIterator` for `[(&Candidate<'tcx>, ProbeResult)]`
     |
     |
1662 |         for (p, _) in &probes[1..] {
     |                       +
1662 |         for (p, _) in &mut probes[1..] {

   Compiling rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
For more information about this error, try `rustc --explain E0277`.
[RUSTC-TIMING] rustc_typeck test:false 5.502
