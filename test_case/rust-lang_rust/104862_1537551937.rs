plain
   Compiling rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
   Compiling rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
   Compiling rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
   Compiling rustc_smir v0.0.0 (/checkout/compiler/rustc_smir)
error[E0004]: non-exhaustive patterns: `&AssertKind::OccupiedNiche { .. }` not covered
    |
155 |     match assert_message {
155 |     match assert_message {
    |           ^^^^^^^^^^^^^^ pattern `&AssertKind::OccupiedNiche { .. }` not covered
    |
note: `AssertKind<rustc_middle::mir::Operand<'_>>` defined here
    |
    |
773 | pub enum AssertKind<O> {
...
...
782 |     OccupiedNiche { found: O, start: O, end: O },
    |     ^^^^^^^^^^^^^ not covered
    = note: the matched value is of type `&AssertKind<rustc_middle::mir::Operand<'_>>`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
189 ~         },
189 ~         },
190 +         &AssertKind::OccupiedNiche { .. } => todo!()

For more information about this error, try `rustc --explain E0004`.
error: could not compile `rustc_smir` (lib) due to previous error
warning: build failed, waiting for other jobs to finish...
