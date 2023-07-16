plain
   Compiling rustc_ast_pretty v0.0.0 (/checkout/compiler/rustc_ast_pretty)
error: unnecessary parentheses around block return value
   --> compiler/rustc_ast_pretty/src/pprust/state.rs:380:38
    |
380 |             ast::StrStyle::Cooked => (format!("\"{}\"", st.escape_debug())),
    |                                      ^                                    ^
    |
    = note: `-D unused-parens` implied by `-D warnings`
help: remove these parentheses
    |
380 -             ast::StrStyle::Cooked => (format!("\"{}\"", st.escape_debug())),
380 +             ast::StrStyle::Cooked => format!("\"{}\"", st.escape_debug()),

error: could not compile `rustc_ast_pretty` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:05:22
