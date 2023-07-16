plain
    Checking rustc_attr v0.0.0 (/checkout/compiler/rustc_attr)
    Checking rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
    Checking rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
error[E0425]: cannot find function, tuple struct or tuple variant `BytePos` in this scope
   --> compiler/rustc_ast_lowering/src/expr.rs:813:70
    |
813 | ...                   fn_decl_span.with_lo(fn_decl_span.lo() + BytePos("static ".len())),
    |
help: consider importing one of these items
    |
1   | use crate::path::BytePos;
