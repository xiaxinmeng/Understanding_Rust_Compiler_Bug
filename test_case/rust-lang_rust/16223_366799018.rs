rust
mod expr {
    struct BoolConst(bool);
    struct Not { child: Box<AnyExpr> }
    struct IfThenElse { children: Box<IfThenElseChildren> }
    // Exists to only have one `Box` indirection instead of one per child for `IfThenElse`.
    struct IfThenElseChildren {
        cond: AnyExpr,
        then_case: AnyExpr,
        else_case: AnyExpr
    }
}
/// A generic super-type covering all existing expression types in a single enum.
enum AnyExpr {
    BoolConst(expr::BoolConst),
    Not(expr::Not),
    IfThenElse(expr::IfThenElse)
}
