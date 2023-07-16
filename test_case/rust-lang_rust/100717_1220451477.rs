rust
> impl<'tcx> SessionDiagnostic for UnusedGenericParams {
>   fn into_diagnostic<T: EmissionGuarantee>( .. ) -> DiagnosticBuilder<'tcx, T> {
>     let mut diag = sess.struct_diagnostic(rustc_errors:..);
>     ..
>   }
> }
> 