
error: this file contains an unclosed delimiter
    --> compiler/rustc_hir_typeck/src/method/suggest.rs:2649:3
     |
38   | impl<'a, 'tcx> FnCtxt<'a, 'tcx> {
     |                                 - unclosed delimiter
...
110  |                                  sugg_span: Span| {
     |                                                   - this delimiter might not be properly closed...
...
240  |         };
     |         - ...as it matches this but it has different indentation
...
2649 | }
     |   ^

error: expected one of `.`, `;`, `?`, `else`, or an operator, found `{`
    --> compiler/rustc_hir_typeck/src/method/suggest.rs:1448:49
     |
1448 |             let self_ty = self.check_expr(expr) {
     |                                                 ^ expected one of `.`, `;`, `?`, `else`, or an operator

error: enum is not supported in `trait`s or `impl`s
    --> compiler/rustc_hir_typeck/src/method/suggest.rs:2557:1
     |
2557 | pub enum SelfSource<'a> {
     | ^^^^^^^^^^^^^^^^^^^^^^^
