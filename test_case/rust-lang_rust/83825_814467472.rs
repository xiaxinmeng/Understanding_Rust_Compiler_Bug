
│ │ │ ├─25ms DEBUG rustc_mir::transform::simplify simplifying bb0
│ │ │ ├─25ms DEBUG rustc_middle::ty::fold HasTypeFlagsVisitor: c=Const { ty: usize, val: Unevaluated(Unevaluated { def: WithOptConstParam { did
: DefId(0:4 ~ issue_64494[317d]::Foo::VAL), const_param_did: None }, substs: [T], promoted: None }) } c.flags=HAS_TY_PARAM | HAS_CT_PROJECTION 
| STILL_FURTHER_SPECIALIZABLE self.flags=HAS_FREE_REGIONS
