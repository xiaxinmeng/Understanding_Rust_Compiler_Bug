
            cx.expr(
                ty.span,
                ast::expr_call(
                    #ast(expr){$(s).emit_from_vec},
                    [#ast(expr){{|__e| $(ser_e)}}],
                    false)))]
