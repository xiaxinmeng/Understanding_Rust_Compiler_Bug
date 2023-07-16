
error[E0282]: type annotations needed
   --> miri/fn_call.rs:391:33
    |
391 | /                                 eval_body(self.tcx.tcx, cid, ty::ParamEnv::reveal_all())
392 | |                                     .ok_or_else(||EvalErrorKind::MachineError("<already reported>".to_string()).into())?.0
    | |________________________________________________________________________________________________________________________^ cannot infer type for `_`
