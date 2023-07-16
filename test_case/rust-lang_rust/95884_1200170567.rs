plain
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
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

    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_typeck` due to 2 previous errors
