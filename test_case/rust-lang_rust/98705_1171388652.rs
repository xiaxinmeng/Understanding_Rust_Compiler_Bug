plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking rustc_error_messages v0.0.0 (/checkout/compiler/rustc_error_messages)
    Checking rustc_target v0.0.0 (/checkout/compiler/rustc_target)
    Checking rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
    Checking rustc_ast_pretty v0.0.0 (/checkout/compiler/rustc_ast_pretty)
error[E0599]: no method named `precedence` found for reference `&rustc_ast::Expr` in the current scope
  --> compiler/rustc_ast_pretty/src/pprust/state/expr.rs:52:47
   |
52 |         self.print_expr_cond_paren(expr, expr.precedence().order() < prec)
   |                                               ^^^^^^^^^^ method not found in `&rustc_ast::Expr`

error[E0599]: no method named `precedence` found for reference `&rustc_ast::Expr` in the current scope
    --> compiler/rustc_ast_pretty/src/pprust/state.rs:1218:64
     |
1218 |         let npals = || parser::needs_par_as_let_scrutinee(expr.precedence().order());
     |                                                                ^^^^^^^^^^ method not found in `&rustc_ast::Expr`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_ast_pretty` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_ast_pretty` due to 2 previous errors
