plain
    Checking rustc_attr v0.0.0 (/checkout/compiler/rustc_attr)
    Checking rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
    Checking rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
error[E0599]: no method named `parse_else_expr` found for mutable reference `&mut Parser<'a>` in the current scope
     |
     |
2517 |             let else_clause = self.parse_else_expr()?;

For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_parse` due to previous error
warning: build failed, waiting for other jobs to finish...
