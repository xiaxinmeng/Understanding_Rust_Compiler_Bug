plain
   Compiling rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
   Compiling rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
   Compiling rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
   Compiling rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
error[E0228]: the lifetime bound for this object type cannot be deduced from context; please supply an explicit bound
    |
331 | /             dyn '_
331 | /             dyn '_
332 | |                 + for<'a, 'b> FnOnce(
333 | |                     &'b mut DiagnosticBuilder<'a, ()>,
334 | |                 ) -> &'b mut DiagnosticBuilder<'a, ()>,

   Compiling rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
   Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
For more information about this error, try `rustc --explain E0228`.
