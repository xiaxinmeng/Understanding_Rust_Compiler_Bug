rust
    /// Print a `let pat = scrutinee` expression.
    crate fn print_let(&mut self, pat: &ast::Pat, scrutinee: &ast::Expr) {
        self.s.word("let ");

        self.print_pat(pat);
        self.s.space();

        self.word_space("=");
        let order = scrutinee.precedence().order();
        self.print_expr_cond_paren(
            scrutinee,
            parser::contains_exterior_struct_lit(scrutinee)
            || parser::needs_par_as_let_scrutinee(order) && order > PREC_JUMP
        )
    }
