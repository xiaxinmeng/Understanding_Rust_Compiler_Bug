rust
fn transform_cond(ite: expr::IfThenElse) -> TransformOutcome {
    if let (&mut AnyExpr::Not(ref mut not), &mut ref mut then_case, &mut ref mut else_case) = ite.as_childs_tuple_mut() {
        use std::mem;
        let not       = mem::replace(not,       unimplemented!());
        let then_case = mem::replace(then_case, unimplemented!());
        let else_case = mem::replace(else_case, unimplemented!());
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
