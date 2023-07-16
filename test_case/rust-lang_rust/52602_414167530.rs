plain
[00:22:46]    Compiling rustc_data_structures v0.0.0 (file:///checkout/src/librustc_data_structures)
[00:22:50]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:22:50]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:22:54]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:23:01] error: found removed `do catch` syntax
[00:23:01]      |
[00:23:01]      |
[00:23:01] 1761 |             let pat_arg: PResult<'a, _> = do catch {
[00:23:01]      |
[00:23:01]      |
[00:23:01]      = help: Following RFC #2388, the new non-placeholder syntax is `try`
[00:23:01] 
[00:23:02] error: unused import: `DiagnosticId`
[00:23:02]    |
[00:23:02]    |
[00:23:02] 47 | use errors::{self, Applicability, DiagnosticBuilder, DiagnosticId};
[00:23:02]    |
[00:23:02]    = note: `-D unused-imports` implied by `-D warnings`
[00:23:02] 
4-unknown-linux-gnu/stage1
