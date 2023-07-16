rust
fn transform_cond(ite: expr::IfThenElse) -> TransformOutcome {
    if let box IfThenElseChilds{ cond: AnyExpr::Not(not), then_case, else_case } = ite.childs {
        return TransformOutcome::transformed(
            expr::IfThenElse::new(
                not.into_single_child(),
                else_case,
                then_case
            )
        )
    }
    TransformOutcome::identity(ite)
}
