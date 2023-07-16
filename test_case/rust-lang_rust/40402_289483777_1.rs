rust
/// Where can patterns ultimately be used?
pub enum PatternSource<'hir> {
    MatchExpr(&'hir hir::Expr),
    LetDecl(&'hir hir::Local),
}

// Note: this impl already exists... I'm just adding a method
impl<'hir> Map<'hir> {
    ...

    /// Determines whether a pattern is located within a `let` statement or a `match` expression.
    ///
    /// Returns `Some(true)` if `id` 
    pub fn get_pattern_source(&self, pat: &hir::Pat) -> PatternSource<'hir> {
        let result = self.walk_parent_nodes(pat.id, |node| match *node {
            NodePat(_) => false, // keep walking as long as we are in a pattern
            _ => true, // stop walking once we exit patterns
        }).expect("never found a parent for the pattern");

        match result {
            NodeExpr(ref e) => {
                // the enclosing expression must be a `match`
                assert!(match e.node { ExprMatch(..) => true, _ => false });
                PatternSource::MatchExpr(e)
            }
            NodeStmt(ref s) => {
                // the enclosing statement must be a `let`
                match s.node {
                    StmtDecl(ref decl, _) => {
                        match decl.kind {
                            DeclLocal(ref local) => Some(PatternSource::LetDecl(decl)),
                            _ => span_bug!(s.span, "expected pattern to be in a `let` statement"),
                        }
                    }
                    _ => span_bug!(s.span, "expected pattern to be in a `let` statement"),
               }
            }
            _ => span_bug!(s.span, "pattern in unexpected location {:?}", result)
      }
}
