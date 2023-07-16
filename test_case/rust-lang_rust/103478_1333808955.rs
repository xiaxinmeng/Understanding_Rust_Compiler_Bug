plain
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
error: unused variable: `ident`
   --> compiler/rustc_parse/src/parser/diagnostics.rs:363:40
    |
363 | ...                   let Ok(ident) = self.sess.source_map().span_to_snippet(self.token.span) {
    |                              ^^^^^ help: if this is intentional, prefix it with an underscore: `_ident`
    |
    = note: `-D unused-variables` implied by `-D warnings`
error: could not compile `rustc_parse` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_parse` due to previous error
Build completed unsuccessfully in 0:03:07
