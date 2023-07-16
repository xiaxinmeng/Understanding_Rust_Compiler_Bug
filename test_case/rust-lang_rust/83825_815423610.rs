
│ │ │ ├─21ms DEBUG rustc_mir::transform::simplify simplifying bb0
│ │ │ ├─21ms DEBUG rustc_middle::ty::fold HasTypeFlagsVisitor: c=Const { ty: usize, val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:4 ~ issue_64494[317d]::Foo::VAL), const_param_did: None }, substs: [T], promoted: None }) } c.flags=HAS_TY_PARAM | HAS_CT_PROJECTION | STILL_FURTHER_SPECIALIZABLE self.flags=HAS_FREE_REGIONS
│ │ │ ├─21ms DEBUG rustc_middle::ty::fold HasTypeFlagsVisitor: c=Const { ty: usize, val: Value(Scalar(0x0000000000000005)) } c.flags=(empty) self.flags=HAS_FREE_REGIONS
│ │ │ ├─21ms DEBUG rustc_middle::ty::fold HasTypeFlagsVisitor: t=bool t.flags=(empty) self.flags=HAS_FREE_REGIONS
│ │ │ ├─21ms DEBUG rustc_middle::ty::fold HasTypeFlagsVisitor: c=Const { ty: usize, val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:4 ~ issue_64494[317d]::Foo::VAL), const_param_did: None }, substs: [T], promoted: None }) } c.flags=HAS_TY_PARAM | HAS_CT_PROJECTION | STILL_FURTHER_SPECIALIZABLE self.flags=HAS_FREE_REGIONS
│ │ │ ├─21ms DEBUG rustc_mir::const_eval::eval_queries eval_body_using_ecx: GlobalId { instance: Instance { def: Item(WithOptConstParam { did: DefId(0:14 ~ issue_64494[317d]::{impl#1}::{constant#0}), const_param_did: Some(DefId(0:9 ~ issue_64494[317d]::Is::T)) }), substs: [^0] }, promoted: None }, ParamEnv { caller_bounds: [], reveal: UserFacing }
│ │ │ ├─21ms DEBUG rustc_middle::ty::fold HasTypeFlagsVisitor: t=bool t.flags=(empty) self.flags=HAS_TY_PARAM | HAS_RE_PARAM | HAS_CT_PARAM | NEEDS_SUBST
│ │ │ ├─21ms DEBUG rustc_middle::ty::normalize_erasing_regions normalize_erasing_regions::<&rustc_middle::ty::TyS>(value=bool, param_env=ParamEnv { caller_bounds: [], reveal: All })
│ │ │ ├─21ms DEBUG rustc_middle::ty::fold HasTypeFlagsVisitor: t=bool t.flags=(empty) self.flags=HAS_FREE_REGIONS | HAS_RE_LATE_BOUND
│ │ │ ├─21ms DEBUG rustc_middle::ty::fold HasTypeFlagsVisitor: t=bool t.flags=(empty) self.flags=HAS_TY_PROJECTION | HAS_TY_OPAQUE | HAS_CT_PROJECTION | HAS_PROJECTION
│ │ │ ├─21ms DEBUG rustc_middle::ty::fold HasTypeFlagsVisitor: t=bool t.flags=(empty) self.flags=HAS_TY_PARAM | HAS_CT_PARAM | HAS_TY_INFER | HAS_CT_INFER | HAS_TY_PLACEHOLDER | HAS_CT_PLACEHOLDER | HAS_FREE_LOCAL_REGIONS | HAS_FREE_LOCAL_NAMES
│ │ │ ├─21ms DEBUG rustc_middle::ty::print::pretty def_path_str: def_id=DefId(0:14 ~ issue_64494[317d]::{impl#1}::{constant#0}), ns=ValueNS
│ │ │ ├─21ms DEBUG rustc_middle::ty::print::pretty try_print_visible_def_path: def_id=DefId(0:14 ~ issue_64494[317d]::{impl#1}::{constant#0})
│ │ │ ├─21ms DEBUG rustc_middle::ty::print default_print_def_path: def_id=DefId(0:14 ~ issue_64494[317d]::{impl#1}::{constant#0}), substs=[]
│ │ │ ├─21ms DEBUG rustc_middle::ty::print default_print_def_path: key=DefKey { parent: Some(DefIndex(12)), disambiguated_data: DisambiguatedDefPathData { data: AnonConst, disambiguator: 0 } }
