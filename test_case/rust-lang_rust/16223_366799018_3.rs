rust
fn transform_cond(ite: expr::IfThenElse) -> TransformOutcome {
    match {ite} {
        expr::IfThenElse{ childs: box IfThenElseChilds{ cond: AnyExpr::Not(not), then_case, else_case }, .. } => {
            return TransformOutcome::transformed(
                expr::IfThenElse::new(
                    not.into_single_child(),
                    else_case,
                    then_case
                )
            )
        }
        ite => TransformOutcome::identity(ite)
    }
}
