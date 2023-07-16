
        ast::StmtExpr(e, _) | ast::StmtSemi(e, _) => {
            bcx = expr::trans_into(cx, e, expr::Ignore);
        }
