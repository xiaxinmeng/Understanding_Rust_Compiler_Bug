
│ │ │ ├┘rustc_middle::mir::interpret::queries::const_eval_resolve param_env=ParamEnv { caller_bounds: [], reveal: UserFacing }, ct=Unevaluated { def: WithOptConstParam { did: DefId(0:14 ~ issue_64494[317d]::{impl#1}::{constant#0}), const_param_did: Some(DefId(0:9 ~ issue_64494[317d]::Is::T)) }, substs: [^0], promoted: None }, span=Some(src/test/ui/const-generics/issues/issue-64494.rs:1:1: 1:1 (#0))
│ │ │ ├─22ms DEBUG rustc_middle::ty::fold HasTypeFlagsVisitor: t=^0 t.flags=(empty) self.flags=HAS_FREE_REGIONS | HAS_RE_LATE_BOUND
│ │ │ ├─22ms DEBUG rustc_middle::ty::fold HasTypeFlagsVisitor: t=^0 t.flags=(empty) self.flags=HAS_TY_PARAM | HAS_CT_PARAM | HAS_TY_INFER | HAS_CT_INFER | HAS_TY_PLACEHOLDER | HAS_CT_PLACEHOLDER | HAS_FREE_LOCAL_REGIONS | HAS_FREE_LOCAL_NAMES
│ │ │ ├─22ms DEBUG rustc_middle::ty::fold HasTypeFlagsVisitor: t=^0 t.flags=(empty) self.flags=HAS_TY_INFER | HAS_RE_INFER | HAS_CT_INFER | NEEDS_INFER
│ │ │ ├┐rustc_middle::mir::interpret::queries::const_eval_resolve param_env=ParamEnv { caller_bounds: [], reveal: UserFacing }, ct=Unevaluated { def: WithOptConstParam { did: DefId(0:14 ~ issue_64494[317d]::{impl#1}::{constant#0}), const_param_did: Some(DefId(0:9 ~ issue_64494[317d]::Is::T)) }, substs: [^0], promoted: None }, span=Some(src/test/ui/const-generics/issues/issue-64494.rs:1:1: 1:1 (#0))
│ │ │ │└┐rustc_middle::mir::interpret::queries::const_eval_resolve param_env=ParamEnv { caller_bounds: [], reveal: All }, ct=Unevaluated { def: WithOptConstParam { did: DefId(0:4 ~ issue_64494[317d]::Foo::VAL), const_param_did: None }, substs: [^0], promoted: None }, span=None
│ │ │ │ ├┐rustc_middle::mir::interpret::queries::const_eval_resolve param_env=ParamEnv { caller_bounds: [], reveal: All }, ct=Unevaluated { def: WithOptConstParam { did: DefId(0:4 ~ issue_64494[317d]::Foo::VAL), const_param_did: None }, substs: [^0], promoted: None }, span=None
│ │ │ │ │└┐rustc_middle::ty::instance::resolve_opt_const_arg param_env=ParamEnv { caller_bounds: [], reveal: All }, def=WithOptConstParam { did: DefId(0:4 ~ issue_64494[317d]::Foo::VAL), const_param_did: None }, substs=[^0]
│ │ │ │ │ ├─0ms DEBUG rustc_middle::ty::fold HasTypeFlagsVisitor: t=^0 t.flags=(empty) self.flags=HAS_FREE_REGIONS | HAS_RE_LATE_BOUND
│ │ │ │ │ ├─0ms DEBUG rustc_middle::ty::fold HasTypeFlagsVisitor: t=^0 t.flags=(empty) self.flags=HAS_TY_PARAM | HAS_CT_PARAM | HAS_TY_INFER | HAS_CT_INFER | HAS_TY_PLACEHOLDER | HAS_CT_PLACEHOLDER | HAS_FREE_LOCAL_REGIONS | HAS_FREE_LOCAL_NAMES
│ │ │ │ │ ├─0ms DEBUG rustc_middle::ty::fold HasTypeFlagsVisitor: t=^0 t.flags=(empty) self.flags=HAS_FREE_REGIONS | HAS_RE_LATE_BOUND
│ │ │ │ │ ├─0ms DEBUG rustc_query_system::query::plumbing ty::query::get_query<resolve_instance>(key=ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All }, value: (DefId(0:4 ~ issue_64494[317d]::Foo::VAL), [^0]) }, span=src/test/ui/const-generics/issues/issue-64494.rs:1:1: 1:1 (#0))
│ │ │ │ │ ├┐rustc_middle::ty::instance::resolve_opt_const_arg param_env=ParamEnv { caller_bounds: [], reveal: All }, def=WithOptConstParam { did: DefId(0:4 ~ issue_64494[317d]::Foo::VAL), const_param_did: None }, substs=[^0]
│ │ │ │ │ │└┐rustc_ty_utils::instance::resolve_instance key=ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All }, value: (DefId(0:4 ~ issue_64494[317d]::Foo::VAL), [^0]) }
│ │ │ │ │ │ ├─0ms DEBUG rustc_query_system::query::plumbing ty::query::get_query<opt_const_param_of>(key=DefId(0:4 ~ issue_64494[317d]::Foo::VAL), span=src/test/ui/const-generics/issues/issue-64494.rs:1:1: 1:1 (#0))
│ │ │ │ │ │ ├─0ms DEBUG rustc_middle::ty::fold HasTypeFlagsVisitor: t=^0 t.flags=(empty) self.flags=HAS_TY_PARAM | HAS_CT_PARAM | HAS_TY_INFER | HAS_CT_INFER | HAS_TY_PLACEHOLDER | HAS_CT_PLACEHOLDER | HAS_FREE_LOCAL_REGIONS | HAS_FREE_LOCAL_NAMES
│ │ │ │ │ │ ├┐rustc_ty_utils::instance::resolve_instance key=ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All }, value: (DefId(0:4 ~ issue_64494[317d]::Foo::VAL), [^0]) }
│ │ │ │ │ │ │└┐rustc_ty_utils::instance::inner_resolve_instance key=ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All }, value: (WithOptConstParam { did: DefId(0:4 ~ issue_64494[317d]::Foo::VAL), const_param_did: None }, [^0]) }
│ │ │ │ │ │ │ ├─0ms DEBUG rustc_query_system::query::plumbing ty::query::get_query<trait_of_item>(key=DefId(0:4 ~ issue_64494[317d]::Foo::VAL), span=src/test/ui/const-generics/issues/issue-64494.rs:1:1: 1:1 (#0))
│ │ │ │ │ │ │ ├─0ms DEBUG rustc_ty_utils::instance  => associated item, attempting to find impl in param_env ParamEnv {
│ │ │ │ │ │ │ │     caller_bounds: [],
│ │ │ │ │ │ │ │     reveal: All,
│ │ │ │ │ │ │ │ }
│ │ │ │ │ │ │ ├─0ms DEBUG rustc_ty_utils::instance resolve_associated_item(trait_item=DefId(0:4 ~ issue_64494[317d]::Foo::VAL), param_env=ParamEnv { caller_bounds: [], reveal: All }, trait_id=DefId(0:3 ~ issue_64494[317d]::Foo), rcvr_substs=[^0])
│ │ │ │ │ │ │ ├─0ms DEBUG rustc_middle::ty::sty Binder::dummy(<^0 as Foo>)
thread 'rustc' panicked at 'assertion failed: !value.has_escaping_bound_vars()', /Users/user/rust/compiler/rustc_middle/src/ty/sty.rs:969:9
