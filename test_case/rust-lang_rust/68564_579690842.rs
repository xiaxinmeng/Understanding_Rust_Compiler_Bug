rust
#[test]
fn single_type() {
    let e = Expr {
        node: ExprKind::B(B { called: false }),
    };

    let mut folder = &mut (&mut LitFold as &mut dyn Fold<B>);

    let e = e.fold_with(&mut folder);

    assert_eq!(
        e,
        Expr {
            node: ExprKind::B(B { called: true }),
        }
    )
}

