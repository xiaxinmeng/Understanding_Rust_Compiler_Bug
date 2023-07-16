rust
wf::obligations(Bar<0>, body_id=HirId(DefId(0:0 ~ lib[b3d6]).0)) =
[
    Obligation(predicate=Binder(WellFormed([(); _]), []), depth=2),
    Obligation(predicate=Binder(WellFormed([(); _]), []), depth=2),
    Obligation(predicate=Binder(WellFormed([(); _]), []), depth=2),
    Obligation(predicate=Binder(WellFormed([(); _]), []), depth=2),
    Obligation(predicate=Binder(WellFormed([(); _]), []), depth=2),
    Obligation(predicate=Binder(WellFormed([(); _]), []), depth=2),
    Obligation(predicate=Binder(WellFormed([(); _]), []), depth=2),
    Obligation(predicate=Binder(WellFormed([(); _]), []), depth=2),
    Obligation(predicate=Binder(WellFormed([(); _]), []), depth=2),
    Obligation(
        predicate=Binder(
            ConstEvaluatable(
                Const {
                    ty: usize,
                    kind: Unevaluated(
                        UnevaluatedConst {
                            def: WithOptConstParam {
                                did: DefId(0:8 ~ lib[b3d6]::Bar::{constant#0}),
                                const_param_did: None
                            },
                            substs: [Const { ty: usize, kind: Value(Leaf(0x0000000000000000)) }]
                        }
                    )
                }
            ),
            []
        ),
        depth=2
    ),
    Obligation(predicate=Binder(ConstEvaluatable(Const { ty: usize, kind: Unevaluated(UnevaluatedConst { def: WithOptConstParam { did: DefId(0:9  ~ lib[b3d6]::Bar::{constant#1}), const_param_did: None }, substs: [Const { ty: usize, kind: Value(Leaf(0x0000000000000000)) }] }) }), []), depth=2),
    Obligation(predicate=Binder(ConstEvaluatable(Const { ty: usize, kind: Unevaluated(UnevaluatedConst { def: WithOptConstParam { did: DefId(0:10 ~ lib[b3d6]::Bar::{constant#2}), const_param_did: None }, substs: [Const { ty: usize, kind: Value(Leaf(0x0000000000000000)) }] }) }), []), depth=2),
    Obligation(predicate=Binder(ConstEvaluatable(Const { ty: usize, kind: Unevaluated(UnevaluatedConst { def: WithOptConstParam { did: DefId(0:11 ~ lib[b3d6]::Bar::{constant#3}), const_param_did: None }, substs: [Const { ty: usize, kind: Value(Leaf(0x0000000000000000)) }] }) }), []), depth=2),
    Obligation(predicate=Binder(ConstEvaluatable(Const { ty: usize, kind: Unevaluated(UnevaluatedConst { def: WithOptConstParam { did: DefId(0:12 ~ lib[b3d6]::Bar::{constant#4}), const_param_did: None }, substs: [Const { ty: usize, kind: Value(Leaf(0x0000000000000000)) }] }) }), []), depth=2),
    Obligation(predicate=Binder(ConstEvaluatable(Const { ty: usize, kind: Unevaluated(UnevaluatedConst { def: WithOptConstParam { did: DefId(0:13 ~ lib[b3d6]::Bar::{constant#5}), const_param_did: None }, substs: [Const { ty: usize, kind: Value(Leaf(0x0000000000000000)) }] }) }), []), depth=2),
    Obligation(predicate=Binder(ConstEvaluatable(Const { ty: usize, kind: Unevaluated(UnevaluatedConst { def: WithOptConstParam { did: DefId(0:14 ~ lib[b3d6]::Bar::{constant#6}), const_param_did: None }, substs: [Const { ty: usize, kind: Value(Leaf(0x0000000000000000)) }] }) }), []), depth=2),
    Obligation(predicate=Binder(ConstEvaluatable(Const { ty: usize, kind: Unevaluated(UnevaluatedConst { def: WithOptConstParam { did: DefId(0:15 ~ lib[b3d6]::Bar::{constant#7}), const_param_did: None }, substs: [Const { ty: usize, kind: Value(Leaf(0x0000000000000000)) }] }) }), []), depth=2),
    Obligation(predicate=Binder(ConstEvaluatable(Const { ty: usize, kind: Unevaluated(UnevaluatedConst { def: WithOptConstParam { did: DefId(0:16 ~ lib[b3d6]::Bar::{constant#8}), const_param_did: None }, substs: [Const { ty: usize, kind: Value(Leaf(0x0000000000000000)) }] }) }), []), depth=2)]
