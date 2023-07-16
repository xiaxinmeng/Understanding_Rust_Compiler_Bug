 rust
           ExprKind::Loop { .. } |
            ExprKind::Block { .. } |
            ExprKind::Break { .. } |
            ExprKind::Continue { .. } |
            ExprKind::Return { .. } =>
                // FIXME(#27840) these probably want their own
                // category, like "nonterminating"
                Some(Category::Rvalue(RvalueFunc::Into)),
