rust
fn transform_cond(ite: expr::IfThenElse) -> TransformOutcome {
    if ite.childs.cond.kind() == ExprKind::Not {
        if let (AnyExpr::Not(not), then_case, else_case) = ite.childs.into_childs_tuple() {
            return TransformOutcome::transformed(
                expr::IfThenElse::new(
                    not.into_single_child(),
                    else_case,
                    then_case
                )
            )
        }
        unreachable!()
    }
    TransformOutcome::identity(ite)
}
