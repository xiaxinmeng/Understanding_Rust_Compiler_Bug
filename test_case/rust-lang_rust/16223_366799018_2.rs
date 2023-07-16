
error[E0382]: use of collaterally moved value: `ite.childs.then_case`
108 |         if let box IfThenElseChilds{ cond: AnyExpr::Not(not), then_case, else_case } = ite.childs {
    |                                                         ---   ^^^^^^^^^ value used here after move
    |                                                         |
    |                                                         value moved here
    |
    = note: move occurs because the value has type `ast2::formulas::not::Not`, which does not implement the `Copy` trait
