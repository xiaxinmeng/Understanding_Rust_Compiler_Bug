plain
    Checking rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
    Checking rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking chalk-engine v0.55.0
error[E0425]: cannot find function, tuple struct or tuple variant `BytePos` in this scope
    |
    |
816 | ...                   fn_decl_span.with_lo(fn_decl_span.lo() + BytePos("static ".len())),
    |
help: consider importing one of these items
    |
1   | use crate::path::BytePos;
