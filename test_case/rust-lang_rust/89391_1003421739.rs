plain
    Checking rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
    Checking rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
error[E0432]: unresolved import `rustc_span::hygiene::ForLoopLoc`
   |
   |
15 | use rustc_span::{hygiene::ForLoopLoc, BytePos, DUMMY_SP};
   |                  ^^^^^^^^^^^^^^^^^^^ no `ForLoopLoc` in `hygiene`
For more information about this error, try `rustc --explain E0432`.
error: could not compile `rustc_ast_lowering` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
